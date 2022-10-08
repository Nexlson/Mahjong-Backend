// @generated automatically by Diesel CLI.

diesel::table! {
    room (id) {
        id -> Int4,
        number_of_participants -> Int4,
        player_1 -> Nullable<Varchar>,
        player_2 -> Nullable<Varchar>,
        player_3 -> Nullable<Varchar>,
        player_4 -> Nullable<Varchar>,
    }
}

diesel::table! {
    round (id) {
        id -> Int4,
        room_id -> Nullable<Int4>,
        player_1 -> Nullable<Int4>,
        player_2 -> Nullable<Int4>,
        player_3 -> Nullable<Int4>,
        player_4 -> Nullable<Int4>,
    }
}

diesel::joinable!(round -> room (room_id));

diesel::allow_tables_to_appear_in_same_query!(room, round,);
