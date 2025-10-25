mod user;
pub use user::*;

use dioxus::prelude::*;
#[cfg(feature = "server")]
use {
    sea_orm::{Database, DatabaseConnection},
    std::sync::{Arc, LazyLock},
    tokio::sync::RwLock,
};

#[cfg(feature = "server")]
#[derive(Clone, Debug, Default)]
pub struct AppServerState {
    db: DatabaseConnection,
}

#[cfg(feature = "server")]
pub static APPSTATE: LazyLock<Arc<RwLock<AppServerState>>> =
    LazyLock::new(|| Arc::new(RwLock::new(AppServerState::default())));

#[server]
pub async fn init_server() -> Result<(), ServerFnError> {
    use dioxus::logger::tracing;
    use migration::{Migrator, MigratorTrait};
    let state = Arc::clone(&APPSTATE);
    let mut state = state.write().await;
    let db = Database::connect("postgres://spoon:mahjong_at_home@localhost:5432/mahjong_at_home")
        .await?;
    tracing::debug!("DONE: connect to db");
    Migrator::up(&db, None).await?;
    tracing::debug!("DONE: migrator up");
    state.db = db;
    drop(state);
    Ok(())
}
