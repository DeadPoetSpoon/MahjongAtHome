use crate::Db;
use crate::schema::{self, mahjong_user};
use diesel::insert_into;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Insertable)]
#[diesel(table_name = schema::mahjong_user)]
pub struct MahjongUser {
    id: u32,
    email: String,
    psd: String,
    token: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = schema::mahjong_user)]
#[serde(crate = "rocket::serde")]
pub struct UserSignupInfo<'r> {
    id: Option<u32>,
    email: &'r str,
    psd: &'r str,
}

#[post("/signup", data = "<sign_up_info>")]
pub async fn signup(
    mut sign_up_info: Json<UserSignupInfo<'_>>,
    mut db: Connection<Db>,
) -> Result<Status, String> {
    let user_ids: QueryResult<i64> = mahjong_user::table
        .filter(mahjong_user::email.eq(sign_up_info.email))
        .count()
        .get_result(&mut db)
        .await;
    let user_ids = if user_ids.is_ok() {
        user_ids.unwrap_or(0)
    } else {
        return Err(user_ids.err().unwrap().to_string());
    };
    if user_ids > 0 {
        return Ok(Status::NotAcceptable);
    }
    let max_id: u32 = mahjong_user::table
        .select(mahjong_user::id)
        .order(mahjong_user::id.desc())
        .first(&mut db)
        .await
        .unwrap_or(0);
    let insert_user_id = max_id + 1;
    sign_up_info.0.id = Some(insert_user_id);
    let insert_id = insert_into(mahjong_user::table)
        .values(&sign_up_info.0)
        .execute(&mut db)
        .await;
    if insert_id.is_err() {
        return Err(insert_id.err().unwrap().to_string());
    }

    Ok(Status::Accepted)
}
