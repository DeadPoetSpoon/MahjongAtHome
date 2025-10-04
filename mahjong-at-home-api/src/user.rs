use crate::Db;
use crate::result::{ApiResult, ApiResultMsg};
use crate::schema::{self, mahjong_user};
use diesel::insert_into;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[db_enum(existing_type_path = "crate::schema::sql_types::Role")]
#[db_enum(value_style = "PascalCase")]
pub enum Role {
    Super,
    Normal,
    Guest,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::mahjong_user)]
pub struct MahjongUser {
    id: i32,
    email: String,
    psd: String,
    role: Role,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub user_email: String,
    pub exp: usize,
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
    pub user_id: i32,
    pub user_email: String,
    pub token: String,
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
        exp: (jsonwebtoken::get_current_timestamp() + 60 * 60 * 24 * 30) as usize,
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
        user_id: user.id,
        user_email: user.email.clone(),
        token,
    };
    ApiResultMsg::new_success_with_data(response).into()
}

#[post("/signup", data = "<sign_up_info>")]
pub async fn signup(
    mut sign_up_info: Json<UserSignupInfo<'_>>,
    mut db: Connection<Db>,
) -> ApiResult<()> {
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
