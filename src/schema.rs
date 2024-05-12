// @generated automatically by Diesel CLI.

diesel::table! {
    user_profile_image (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        image_url -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
        created_at -> Text,
    }
}

diesel::joinable!(user_profile_image -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_profile_image,
    users,
);
