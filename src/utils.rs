// pub mod models;
// pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{NewRoom, NewPlayer, NewRound};
use serde::Deserialize;

use crate::schema::room;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct PlayerRequest {
    nickname: String,
    host: i32,
    score: i32,
    active: i32
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_room(conn: &mut SqliteConnection, player_one: i32) -> usize{

    let new_room = NewRoom {player_one, players: 1};

    diesel::insert_into(room::table)
        .values(&new_room)
        .execute(conn)
        .expect("Error creating new room")
}

pub fn create_player(conn: &mut SqliteConnection, nickname: String, host: i32, score: i32, active: i32) -> usize{
    use crate::schema::player;

    let new_player = NewPlayer {nickname, host, score, active};

    diesel::insert_into(player::table)
        .values(&new_player)
        .execute(conn)
        .expect("Error creating new player")
}

pub fn create_round(conn: &mut SqliteConnection, room_id: i32, active: i32) -> usize{
    use crate::schema::round;

    let new_round = NewRound {room_id, active};

    diesel::insert_into(round::table)
        .values(&new_round)
        .execute(conn)
        .expect("Error creating new round")
}

pub fn join_room(conn: &mut SqliteConnection, room_id:i32, player_id: i32) -> usize{
    use crate::schema::room::dsl::*;

    // check players insert into empty player
    let number_of_player = room.select(players).filter(id.eq(room_id))
        .first::<i32>(conn).expect("Cannot load players");

    if number_of_player == 1 {
        diesel::update(room.filter(id.eq(room_id)))
        .set(player_two.eq(player_id))
        .execute(conn)
        .expect("Error joining room")
    } else if number_of_player == 2 {
        diesel::update(room.filter(id.eq(room_id)))
        .set(player_three.eq(player_id))
        .execute(conn)
        .expect("Error joining room")
    } else if (number_of_player == 3) {
        diesel::update(room.filter(id.eq(room_id)))
        .set(player_four.eq(player_id))
        .execute(conn)
        .expect("Error joining room")
    } else {
        0 // false
    }
}