use crate::Db;
use crate::result::{ApiResult, ApiResultMsg};
use crate::schema::{self, mahjong_user, user_role};
use rocket::State;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::tokio::sync::RwLock;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::insert_into;
use rocket_db_pools::diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Token(String);
#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = TokenError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(token: &str) -> bool {
            token != ""
        }
        match request.headers().get_one("Authorization") {
            None => Outcome::Error((Status::BadRequest, TokenError::Missing)),
            Some(token) if is_valid(token) => Outcome::Success(Token(token.to_string())),
            Some(_) => Outcome::Error((Status::BadRequest, TokenError::Invalid)),
        }
    }
}

#[derive(Debug, diesel_derive_enum::DbEnum, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[db_enum(existing_type_path = "crate::schema::sql_types::Role")]
#[db_enum(value_style = "PascalCase")]
pub enum Role {
    Super,
    Normal,
    Guest,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::user_role)]
pub struct UserRole {
    name: Role,
    allowed: Vec<Option<String>>,
    excepted: Vec<Option<String>>,
}

pub type UserRoleCacheHandle = Arc<RwLock<UserRoleCache>>;

pub struct UserRoleCache {
    roles: HashMap<Role, (Vec<String>, Vec<String>)>,
    need_update: bool,
}

impl UserRoleCache {
    pub fn new() -> UserRoleCacheHandle {
        Arc::new(RwLock::new(UserRoleCache {
            roles: HashMap::new(),
            need_update: true,
        }))
    }
    pub fn need_update(&self) -> bool {
        self.need_update
    }
    pub async fn update(&mut self, db: &mut Connection<Db>) {
        if self.need_update {
            let roles: QueryResult<Vec<UserRole>> = user_role::table
                .select(user_role::all_columns)
                .get_results(db)
                .await;
            if roles.is_err() {
                return;
            }
            let roles = roles.unwrap();
            for role in roles {
                self.roles.insert(
                    role.name.clone(),
                    (
                        role.allowed
                            .iter()
                            .map(|x| x.clone().unwrap_or_default())
                            .collect(),
                        role.excepted
                            .iter()
                            .map(|x| x.clone().unwrap_or_default())
                            .collect(),
                    ),
                );
            }
            self.need_update = false;
        }
    }
    pub fn validate(&self, role: &Role, action: &str) -> bool {
        if let Some((allowed, excepted)) = self.roles.get(role) {
            let mut is_allowed = false;
            let mut is_excepted = false;
            let all = &"*".to_string();
            if allowed.contains(all) {
                is_allowed = true;
            } else {
                if allowed.iter().any(|x| action.starts_with(x)) {
                    is_allowed = true;
                }
            }
            if excepted.contains(all) {
                is_excepted = true;
            } else {
                if excepted.iter().any(|x| action.starts_with(x)) {
                    is_excepted = true;
                }
            }
            is_allowed && !is_excepted
        } else {
            false
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::mahjong_user)]
pub struct MahjongUser {
    id: i32,
    email: String,
    psd: String,
    role: Role,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    user_id: i32,
    user_email: String,
    user_role: Role,
    exp: usize,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::mahjong_user)]
#[serde(crate = "rocket::serde")]
pub struct UserSignupInfo<'r> {
    email: &'r str,
    psd: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLoginResponse {
    id: i32,
    email: String,
    token: String,
}

#[post("/login", data = "<login_info>")]
pub async fn login(
    login_info: Json<UserSignupInfo<'_>>,
    mut db: Connection<Db>,
) -> ApiResult<UserLoginResponse> {
    let user: QueryResult<MahjongUser> = mahjong_user::table
        .filter(mahjong_user::email.eq(login_info.email))
        .first(&mut db)
        .await;
    if user.is_err() {
        return ApiResultMsg::new_failure_with_msg(format!("{:?}", user.err().unwrap())).into();
    }
    let user = user.unwrap();
    let pass = password_auth::verify_password(login_info.psd.as_bytes(), &user.psd);
    if pass.is_err() {
        return ApiResultMsg::new_failure_with_msg("密码错误").into();
    }
    let claims = Claims {
        user_id: user.id,
        user_email: user.email.clone(),
        user_role: user.role,
        // exp: (jsonwebtoken::get_current_timestamp() + 60 * 60 * 24 * 30) as usize,
        exp: (jsonwebtoken::get_current_timestamp() + 10) as usize,
    };
    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("MahjongAtHome".as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => return ApiResultMsg::new_failure_with_msg("生成token失败").into(),
    };
    let response = UserLoginResponse {
        id: user.id,
        email: user.email.clone(),
        token,
    };
    ApiResultMsg::new_success_with_data(response).into()
}

#[post("/signup", data = "<sign_up_info>")]
pub async fn signup(
    mut sign_up_info: Json<UserSignupInfo<'_>>,
    token: Token,
    user_role_cache: &State<UserRoleCacheHandle>,
    mut db: Connection<Db>,
) -> ApiResult<()> {
    let validate_token_result =
        validate_token("/api/user/signup", &token, user_role_cache, &mut db).await;
    if !validate_token_result.is_success() {
        return validate_token_result.into();
    }
    let user_ids: QueryResult<i64> = mahjong_user::table
        .filter(mahjong_user::email.eq(sign_up_info.email))
        .count()
        .get_result(&mut db)
        .await;
    let user_ids = if user_ids.is_ok() {
        user_ids.unwrap_or(0)
    } else {
        return ApiResultMsg::new_failure_with_msg(format!("{:?}", user_ids.err().unwrap())).into();
    };
    if user_ids > 0 {
        return ApiResultMsg::new_failure_with_msg(format!("{} 用户已存在", sign_up_info.email))
            .into();
    }

    let psd = password_auth::generate_hash(sign_up_info.psd.as_bytes());
    sign_up_info.psd = psd.as_str();

    let insert_id = insert_into(mahjong_user::table)
        .values(&sign_up_info.0)
        .execute(&mut db)
        .await;
    if insert_id.is_err() {
        return ApiResultMsg::new_failure_with_msg(format!("{:?}", insert_id.err().unwrap()))
            .into();
    }

    ApiResultMsg::default().into()
}

#[get("/auth/<path..>")]
pub async fn auth(
    path: PathBuf,
    token: Token,
    user_role_cache: &State<UserRoleCacheHandle>,
    mut db: Connection<Db>,
) -> ApiResult<()> {
    validate_token(path.to_str().unwrap(), &token, user_role_cache, &mut db)
        .await
        .into()
}
pub async fn validate_token(
    action: &str,
    token: &Token,
    user_role_cache: &State<UserRoleCacheHandle>,
    db: &mut Connection<Db>,
) -> ApiResultMsg<()> {
    let token = jsonwebtoken::decode::<Claims>(
        &token.0,
        &jsonwebtoken::DecodingKey::from_secret("MahjongAtHome".as_ref()),
        &jsonwebtoken::Validation::default(),
    );
    if token.is_err() {
        let err = token.err().unwrap();
        if err
            .kind()
            .eq(&jsonwebtoken::errors::ErrorKind::ExpiredSignature)
        {
            return ApiResultMsg::default()
                .with_code(crate::result::ApiResultCode::TokenExpired)
                .clone();
        } else {
            return ApiResultMsg::new_failure_with_msg(format!("InvalidToken: {}", err));
        }
    }
    let claims = token.unwrap().claims;
    let u_cache_lock = user_role_cache.inner().clone();
    let u_cache = u_cache_lock.read().await;
    if u_cache.need_update() {
        drop(u_cache);
        let u_cache_lock = user_role_cache.inner().clone();
        let mut u_cache = u_cache_lock.write().await;
        u_cache.update(db).await;
        drop(u_cache);
    }
    let u_cache_lock = user_role_cache.inner().clone();
    let u_cache = u_cache_lock.read().await;
    if u_cache.validate(&claims.user_role, action) {
        ApiResultMsg::new_success()
    } else {
        ApiResultMsg::default()
            .with_code(crate::result::ApiResultCode::NotAllowed)
            .clone()
    }
}
