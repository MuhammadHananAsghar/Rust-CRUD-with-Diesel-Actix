// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
        created_at -> Text,
    }
}
