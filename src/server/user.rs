use crate::models::user::UserLoginInfo;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use {
    crate::entities::user, dioxus::logger::tracing, sea_orm::ActiveModelTrait,
    sea_orm::ActiveValue::Set, std::sync::Arc,
};

#[server]
pub async fn login_server(info: UserLoginInfo) -> Result<(), ServerFnError> {
    tracing::debug!("User login: {:?}", info.username);
    let user = user::ActiveModel {
        username: Set(info.username),
        password: Set(info.password),
        ..Default::default()
    };
    let app_state = Arc::clone(&super::APPSTATE);
    let app_state = app_state.read().await;
    user.insert(&app_state.db).await?;
    tracing::debug!("Login finish");
    Ok(())
}
