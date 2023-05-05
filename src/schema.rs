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
        game -> Nullable<Text>,
    }
}

diesel::table! {
    tic_tac_toe_games (id) {
        id -> Text,
        player_1 -> Text,
        player_2 -> Nullable<Text>,
        turn -> Nullable<Text>,
        game_status -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    tic_tac_toe_moves (id) {
        id -> Text,
        player_id -> Text,
        game_id -> Text,
        row_number -> Integer,
        column_number -> Integer,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        phone -> Text,
        web_socket_session -> Text,
        created_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    room_to_users,
    rooms,
    tic_tac_toe_games,
    tic_tac_toe_moves,
    users,
);
