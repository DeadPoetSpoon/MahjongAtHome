// @generated automatically by Diesel CLI.

diesel::table! {
    mahjong_user (id) {
        id -> Oid,
        email -> Text,
        psd -> Text,
        token -> Nullable<Text>,
    }
}
