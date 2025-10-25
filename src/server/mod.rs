mod user;
pub use user::*;

use dioxus::prelude::*;
#[cfg(feature = "server")]
use {
    crate::entities::{
        prelude::*,
        role,
        sea_orm_active_enums::{RoleActionType, RoleType},
    },
    dioxus::logger::tracing,
    sea_orm::{
        ActiveEnum, ColumnTrait, Database, DatabaseConnection, EntityTrait, PaginatorTrait,
        QueryFilter,
    },
    std::sync::{Arc, LazyLock},
    tokio::sync::RwLock,
};

#[cfg(feature = "server")]
#[derive(Clone, Debug, Default)]
pub struct AppServerState {
    init_finished: bool,
    secret_key: String,
    db: DatabaseConnection,
    token_exp_after: usize,
}

#[cfg(feature = "server")]
impl AppServerState {
    pub async fn init_from_config(
        &mut self,
        config: AppInitServerConfig,
    ) -> Result<(), ServerFnError> {
        let default_config = AppInitServerConfig::default();
        if config.token_exp_after.is_some() {
            self.token_exp_after = config.token_exp_after.unwrap();
            tracing::debug!("INIT: use config token expiration");
        } else {
            self.token_exp_after = default_config.token_exp_after.unwrap();
            tracing::debug!("INIT: use default token expiration");
        }
        if config.secret_key.is_some() {
            self.secret_key = config.secret_key.unwrap();
            tracing::debug!("INIT: use config secret key");
        } else {
            self.secret_key = default_config.secret_key.unwrap();
            tracing::debug!("INIT: use default secret key");
        }
        let db_url = if config.db_url.is_some() {
            tracing::debug!("INIT: use config db url");
            config.db_url.unwrap()
        } else {
            tracing::debug!("INIT: use default db url");
            default_config.db_url.unwrap()
        };
        let db = Database::connect(db_url).await?;
        tracing::debug!("INIT: connected to db");

        use migration::{Migrator, MigratorTrait};

        Migrator::up(&db, None).await?;
        tracing::debug!("INIT: migrator up finished");

        {
            use crate::entities::{prelude::*, sea_orm_active_enums::RoleType, user};
            use sea_orm::{
                ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
            };
            let count = User::find()
                .filter(user::Column::Role.eq(RoleType::SuperAdmin))
                .count(&db)
                .await?;
            if count == 0 {
                tracing::debug!("INIT: found no super admin user");
                let username = if config.super_username.is_some() {
                    tracing::debug!("INIT: use config super admin username");
                    config.super_username.unwrap()
                } else {
                    tracing::debug!("INIT: use default super admin username");
                    default_config.super_username.unwrap()
                };
                let password = if config.super_password.is_some() {
                    tracing::debug!("INIT: use config super admin password");
                    config.super_password.unwrap()
                } else {
                    tracing::debug!("INIT: use default super admin password");
                    default_config.super_password.unwrap()
                };

                let password = format!("{}_{}", &password, &self.secret_key);
                let password = password_auth::generate_hash(password.as_bytes());

                let user = user::ActiveModel {
                    username: Set(username),
                    password: Set(password),
                    role: Set(RoleType::SuperAdmin),
                    ..Default::default()
                };
                user.insert(&db).await?;
                tracing::debug!("INIT: created super admin user");
            } else {
                tracing::debug!("INIT: found super admin user");
            }
        }

        self.db = db;
        self.init_finished = true;

        tracing::debug!("INIT: AppServerState Init Finished: {:?}", self);
        Ok(())
    }
}

#[cfg(feature = "server")]
#[derive(serde::Deserialize)]
pub struct AppInitServerConfig {
    secret_key: Option<String>,
    db_url: Option<String>,
    super_username: Option<String>,
    super_password: Option<String>,
    token_exp_after: Option<usize>,
}

#[cfg(feature = "server")]
impl Default for AppInitServerConfig {
    fn default() -> Self {
        Self {
            secret_key: Some("mahjong_is_fun".to_string()),
            db_url: Some("postgres://spoon:mahjong_at_home@localhost/mahjong_at_home".to_string()),
            super_username: Some("admin@math.com".to_string()),
            super_password: Some("mahjong_is_fun".to_string()),
            token_exp_after: Some(0 + 0 * 60 + 0 * 60 * 60 + 1 * 60 * 60 * 24),
        }
    }
}

#[cfg(feature = "server")]
impl AppInitServerConfig {
    pub fn from_file(path: &str) -> Result<Self, ServerFnError> {
        let config_str = std::fs::read_to_string(path)?;
        let config: AppInitServerConfig = toml::from_str(&config_str)?;
        tracing::debug!("INIT: Read init config from file");

        Ok(config)
    }
}

#[cfg(feature = "server")]
pub static APPSTATE: LazyLock<Arc<RwLock<AppServerState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(AppServerState::default())));

#[cfg(feature = "server")]
fn decode_token(token: &str, secret_key: &str) -> Result<Claims, ServerFnError> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret_key.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )?;
    let claims = token_data.claims;
    Ok(claims)
}

#[cfg(feature = "server")]
async fn verify_permission(
    role: &RoleType,
    action: &RoleActionType,
    db: &DatabaseConnection,
) -> Result<(), ServerFnError> {
    if role == &RoleType::SuperAdmin {
        return Ok(());
    }

    let count = Role::find()
        .filter(role::Column::Type.eq(role.to_value()))
        .filter(role::Column::AllowAction.eq(action.to_value()))
        .count(db)
        .await?;
    if count == 0 {
        Err(ServerFnError::ServerError("Permission denied".to_string()))
    } else {
        Ok(())
    }
}

#[server]
pub async fn init_server() -> Result<(), ServerFnError> {
    let state = Arc::clone(&APPSTATE);
    let mut state = state.write().await;

    if state.init_finished {
        return Ok(());
    }

    let config_path = "math.toml";
    let config = if std::fs::exists(config_path)? {
        tracing::debug!("math.toml found, read config from file");
        AppInitServerConfig::from_file(config_path)?
    } else {
        tracing::debug!("no math.toml found, use default config");
        Default::default()
    };

    state.init_from_config(config).await?;

    Ok(())
}

#[server]
pub async fn has_init_server() -> Result<bool, ServerFnError> {
    let state = Arc::clone(&APPSTATE);
    let state = state.read().await;
    Ok(state.init_finished)
}
