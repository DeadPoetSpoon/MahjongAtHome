//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

pub mod user;

#[cfg(feature = "server")]
mod entities;

#[cfg(feature = "server")]
use {
    dioxus::fullstack::{extract::FromRef, FullstackContext},
    dioxus::logger::tracing,
    dioxus::Result,
    sea_orm::{Database, DatabaseConnection},
};

#[cfg(feature = "server")]
#[derive(Clone, Debug, Default)]
pub struct AppServerState {
    init_finished: bool,
    secret_key: String,
    db: DatabaseConnection,
    token_exp_after: usize,
    session_id: uuid::Uuid,
}
#[cfg(feature = "server")]
impl AppServerState {
    pub async fn init_from_config(&mut self, config: AppInitServerConfig) -> Result<()> {
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
impl FromRef<FullstackContext> for AppServerState {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<AppServerState>().unwrap()
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
    pub fn from_file(path: &str) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config: AppInitServerConfig = toml::from_str(&config_str)?;
        tracing::debug!("INIT: Read init config from file");

        Ok(config)
    }
}
