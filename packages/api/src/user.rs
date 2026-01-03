use dioxus::fullstack::{extract::State, SetCookie, SetHeader};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use {
    crate::entities::{
        prelude::*,
        sea_orm_active_enums::{RoleActionType, RoleType},
        user,
    },
    crate::{AppServerState, Claims},
    dioxus::fullstack::{Cookie, TypedHeader},
    dioxus::logger::tracing,
    sea_orm::{
        ActiveEnum, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
    },
};

#[derive(Deserialize, Serialize)]
pub struct UserLoginInfo {
    username: String,
    password: String,
}

#[post("/api/login", state: State<AppServerState>)]
pub async fn login(info: UserLoginInfo) -> Result<SetHeader<SetCookie>> {
    tracing::debug!("User try to login: {:?}", info.username);
    let user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(&info.username))
        .one(&state.db)
        .await?;

    if user.is_none() {
        tracing::debug!("User do not exists.");
        HttpError::not_found("User not found")?;
    }

    let user = user.unwrap();

    let password = format!("{}_{}", &info.password, &state.secret_key);
    let password_verified = password_auth::verify_password(password, &user.password);
    if password_verified.is_err() {
        tracing::debug!("Password is incorrect.");
        HttpError::unauthorized("Password is incorrect")?;
    }
    let claims = Claims {
        id: user.id,
        role: user.role.into_value().to_string(),
        exp: jsonwebtoken::get_current_timestamp() as usize + state.token_exp_after,
        session_id: state.session_id,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(state.secret_key.as_bytes()),
    );
    if token.is_err() {
        tracing::error!("Failed to encode token: {:?}", token.clone().err());
        HttpError::internal_server_error("Failed to encode token")?
    }
    tracing::debug!("Login finished");
    Ok(SetHeader::new(format!("math-token={};", token.unwrap()))?)
}

#[derive(Deserialize, Serialize)]
pub struct UserSignupInfo {
    username: String,
    password: String,
    role: String,
}

#[post("/api/signup", state: State<AppServerState>, header: TypedHeader<Cookie>)]
pub async fn signup(info: UserSignupInfo) -> Result<()> {
    let token = header
        .get("math-token")
        .or_unauthorized("Missing math-token cookie")?;
    let claims = super::decode_token(token, &state.secret_key)?;
    super::verify_permission(&claims.get_role()?, &RoleActionType::Signup, &state.db).await?;
    tracing::debug!("{} try to signup: {:?}", claims.id, info.username);
    let password_attach_key = format!("{}_{}", &info.password, &state.secret_key);
    let password = password_auth::generate_hash(password_attach_key.as_bytes());
    let role_type = RoleType::try_from_value(&info.role)?;
    let user = user::ActiveModel {
        username: Set(info.username),
        password: Set(password),
        role: Set(role_type),
        ..Default::default()
    };
    user.insert(&state.db).await?;
    tracing::debug!("Signup User finished");
    Ok(())
}
