// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Text,
        room_id -> Text,
        user_id -> Text,
        content -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    room_to_users (id) {
        id -> Text,
        room -> Text,
        user -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        phone -> Text,
        created_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    room_to_users,
    rooms,
    users,
);
