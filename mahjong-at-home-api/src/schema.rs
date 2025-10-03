// @generated automatically by Diesel CLI.

diesel::table! {
    mahjong_user (id) {
        id -> Int4,
        email -> Text,
        psd -> Text,
        token -> Nullable<Text>,
    }
}
