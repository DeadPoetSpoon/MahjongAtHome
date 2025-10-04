// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    mahjong_user (id) {
        id -> Int4,
        email -> Text,
        psd -> Text,
        role -> Role,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    user_role (name) {
        name -> Role,
        allowed -> Nullable<Array<Nullable<Text>>>,
        excepted -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(mahjong_user, user_role,);
