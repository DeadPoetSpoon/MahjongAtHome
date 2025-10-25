use crate::models::user::{UserInfo, UserLoginInfo};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use {
    crate::entities::{
        prelude::*,
        sea_orm_active_enums::{RoleActionType, RoleType},
        user,
    },
    dioxus::logger::tracing,
    sea_orm::{
        ActiveEnum, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
    },
    std::sync::Arc,
};

#[cfg(feature = "server")]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Claims {
    id: i32,
    username: String,
    role: String,
    exp: usize,
}
#[cfg(feature = "server")]
impl Claims {
    fn get_role(&self) -> Result<RoleType, ServerFnError> {
        Ok(RoleType::try_from_value(&self.role)?)
    }
}

#[server]
pub async fn login_server(info: UserLoginInfo) -> Result<UserInfo, ServerFnError> {
    tracing::debug!("User try to login: {:?}", info.username);
    let app_state = Arc::clone(&super::APPSTATE);
    let app_state = app_state.read().await;

    let user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(&info.username))
        .one(&app_state.db)
        .await?;

    if user.is_none() {
        tracing::debug!("User do not exists.");
        return Err(ServerFnError::ServerError("User not found".to_string()));
    }

    let user = user.unwrap();

    let password = format!("{}_{}", &info.password, &app_state.secret_key);
    password_auth::verify_password(password, &user.password)?;

    let role_str = user.role.into_value().to_string();
    let claims = Claims {
        id: user.id,
        username: user.username,
        role: role_str.clone(),
        exp: jsonwebtoken::get_current_timestamp() as usize + app_state.token_exp_after,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(app_state.secret_key.as_bytes()),
    )?;
    let user_info = UserInfo {
        id: claims.id,
        username: claims.username,
        role: role_str,
        token,
    };

    tracing::debug!("Login finished");
    Ok(user_info)
}

#[server]
pub async fn signup_server(info: UserLoginInfo) -> Result<(), ServerFnError> {
    if info.token.is_none() {
        return Err(ServerFnError::MissingArg("Token is required".to_string()));
    }

    let app_state = Arc::clone(&super::APPSTATE);
    let app_state = app_state.read().await;

    let claims = super::decode_token(info.token.as_ref().unwrap(), &app_state.secret_key)?;

    super::verify_permission(&claims.get_role()?, &RoleActionType::Signup, &app_state.db).await?;

    tracing::debug!("{} try to signup: {:?}", claims.username, info.username);

    let password_attach_key = format!("{}_{}", &info.password, &app_state.secret_key);
    let password = password_auth::generate_hash(password_attach_key.as_bytes());

    let user = user::ActiveModel {
        username: Set(info.username),
        password: Set(password),
        role: Set(RoleType::User),
        ..Default::default()
    };

    user.insert(&app_state.db).await?;
    tracing::debug!("Signup User finished");
    Ok(())
}
