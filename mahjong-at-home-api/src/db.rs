use rocket_db_pools::Database;
use rocket_db_pools::diesel::PgPool;

#[derive(Database)]
#[database("mahjong_at_home")]
pub struct Db(PgPool);
