// @generated automatically by Diesel CLI.

diesel::table! {
    player (id) {
        id -> Nullable<Integer>,
        nickname -> Text,
        host -> Integer,
        score -> Nullable<Integer>,
    }
}

diesel::table! {
    room (id) {
        id -> Nullable<Integer>,
        player_one -> Nullable<Integer>,
        player_two -> Nullable<Integer>,
        player_three -> Nullable<Integer>,
        player_four -> Nullable<Integer>,
    }
}

diesel::table! {
    round (id) {
        id -> Nullable<Integer>,
        winning_player -> Nullable<Integer>,
        losing_player -> Nullable<Integer>,
        room_id -> Nullable<Integer>,
    }
}

diesel::joinable!(round -> room (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    player,
    room,
    round,
);
