use diesel::prelude::*;
use crate::schema::{room, player, round};

#[derive(Queryable)]
pub struct Player {
    pub id: Option<i32>,
    pub nickname: String,
    pub host: i32,
    pub score: i32,
    pub active: i32
}

#[derive(Queryable)]
pub struct Room {
    pub id: Option<i32>,
    pub player_one: i32,
    pub player_two: Option<i32>,
    pub player_three: Option<i32>,
    pub player_four: Option<i32>,
    pub players: i32
}

#[derive(Queryable)]
pub struct Round {
    pub id: Option<i32>,
    pub winning_player: Option<i32>,
    pub losing_player: Option<i32>,
    pub room_id: i32,
    pub active: i32
}

#[derive(Insertable)]
#[diesel(table_name = room)]
pub struct NewRoom {
    pub player_one: i32,
    pub players: i32
}

#[derive(Insertable)]
#[diesel(table_name = player)]
pub struct NewPlayer {
    pub nickname: String,
    pub host: i32,
    pub score: i32,
    pub active: i32
}

#[derive(Insertable)]
#[diesel(table_name = round)]
pub struct NewRound {
    pub room_id: i32,
    pub active: i32
}