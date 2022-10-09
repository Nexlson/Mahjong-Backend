// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Varchar,
        duration -> Nullable<Time>,
        players -> Array<Nullable<Text>>,
        joined -> Int4,
        number_of_player -> Int4,
    }
}

diesel::table! {
    player (id) {
        id -> Varchar,
        name -> Varchar,
        gameid -> Nullable<Varchar>,
        score -> Int4,
    }
}

diesel::table! {
    round (id) {
        id -> Varchar,
        gameid -> Nullable<Varchar>,
        winning_player -> Nullable<Array<Nullable<Text>>>,
        losing_player -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::joinable!(player -> game (gameid));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    player,
    round,
);
