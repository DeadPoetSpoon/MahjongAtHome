mod db;
mod result;
mod schema;
mod user;

use rocket_db_pools::Database;

use crate::{db::Db, user::UserRoleCache};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/api/user", routes![user::signup, user::login])
        .mount("/", routes![user::auth])
        .manage(UserRoleCache::new())
}
