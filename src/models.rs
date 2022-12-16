use diesel::prelude::*;

#[derive(Queryable)]
pub struct Player {
    pub id: Option<i32>,
    pub nickname: String,
    pub host: i32,
    pub score: Option<i32>
}

#[derive(Queryable)]
pub struct Room {
    pub id: Option<i32>,
    pub player_one: Option<i32>,
    pub player_two: Option<i32>,
    pub player_three: Option<i32>,
    pub player_four: Option<i32>,
}

#[derive(Queryable)]
pub struct Round {
    pub id: Option<i32>,
    pub winning_player: Option<i32>,
    pub losing_player: Option<i32>,
    pub room_id: Option<i32>,
}