use super::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewRoom, NewRound, Room, Round};
use std::env;

pub struct RoomInfo {
    room_id: i32,
    success: bool,
}

pub struct RoundInfo {
    round_id: i32,
    success: bool,
}

// create a room
pub fn create_room(name: String, num_participant: i32) -> RoomInfo {
    use schema::room;

    // establish connection to database
    let connection = &mut establish_connection();

    // create new row
    let new_room = NewRoom {
        number_of_participants: num_participant,
        player_1: Some(name),
        player_2: None,
        player_3: None,
        player_4: None,
    };

    // insert into db
    let inserted_row: Room = diesel::insert_into(room::table)
        .values(&new_room)
        .get_result(connection)
        .expect("Error creating new room");

    // return roomInfo
    RoomInfo {
        room_id: inserted_row.id,
        success: true,
    }
}

// create room
pub fn create_round(room_id: i32) -> RoundInfo {
    use schema::round;

    // establish connection to database
    let connection = &mut establish_connection();

    // retrieve old data if not create new data

    // create new row
    let round_data = NewRound {
        room_id: room_id,
        player_1: 0,
        player_2: 0,
        player_3: 0,
        player_4: 0,
    };

    let inserted_row: Round = diesel::insert_into(round::table)
        .values(&round_data)
        .get_result(connection)
        .expect("Error creating new round");

    RoundInfo {
        round_id: inserted_row.id,
        success: true,
    }
}

pub fn join_room() {
    // add player data into db

}

pub fn update_score() {
    // get player name + score

    // update player data on table
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}