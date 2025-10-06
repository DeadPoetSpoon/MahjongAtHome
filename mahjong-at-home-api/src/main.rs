mod db;
mod result;
mod schema;
mod user;

use crate::{db::Db, user::UserRoleCache};
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let db_config_url = figment
        .extract_inner::<String>("databases.mahjong_at_home.url")
        .expect("Failed to extract database configuration");
    use diesel::Connection;
    let mut conn = diesel::prelude::PgConnection::establish(&db_config_url)
        .expect("Failed to establish database connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    rocket
        .attach(Db::init())
        .mount("/api", routes![user::auth])
        .mount("/api/user", routes![user::signup, user::login])
        .manage(UserRoleCache::new())
}
