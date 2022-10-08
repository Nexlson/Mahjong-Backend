use crate::schema::{room, round};
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Room {
    pub id: i32,
    pub number_of_participants: i32,
    pub player_1: Option<String>,
    pub player_2: Option<String>,
    pub player_3: Option<String>,
    pub player_4: Option<String>,
}

#[derive(Queryable)]
pub struct Round {
    pub id: i32,
    pub room_id: Option<i32>,
    pub player_1: Option<i32>,
    pub player_2: Option<i32>,
    pub player_3: Option<i32>,
    pub player_4: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name=room)]
pub struct NewRoom {
    pub number_of_participants: i32,
    pub player_1: Option<String>,
    pub player_2: Option<String>,
    pub player_3: Option<String>,
    pub player_4: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=round)]
pub struct NewRound {
    pub room_id: i32,
    pub player_1: i32,
    pub player_2: i32,
    pub player_3: i32,
    pub player_4: i32,
}
