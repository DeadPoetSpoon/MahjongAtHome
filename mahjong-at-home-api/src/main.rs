mod db;
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
        .mount("/user", routes![user::signup])
}
