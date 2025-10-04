mod db;
mod result;
mod schema;
mod user;

use rocket_db_pools::Database;

use crate::db::Db;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/api/user", routes![user::signup, user::login])
}
