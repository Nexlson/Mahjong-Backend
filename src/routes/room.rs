use rocket::serde::json::{Json, json, Value};
use serde::Deserialize;
// use crate::utils;

#[derive (Debug, PartialEq, Eq, Deserialize)]
pub struct CreateGameData {
    name: String,
    number_of_participants: i32
}

#[derive (Debug, PartialEq, Eq, Deserialize)]
pub struct JoinGameData {
    name: String,
    room_id: String
}


// Create a game 
#[post("/creategame", format="json", data="<create_game_data>")]
pub fn create_game(create_game_data: Json<CreateGameData>) -> Value{
    // get host name
    let host_name = create_game_data.name.clone();

    // create a room
    // utils::create_room(host_name, create_game_data.number_of_participants);

    // return json
    json!({"status": "ok", "number": create_game_data.number_of_participants})
}

// Join a game
#[post("/joingame", format="json", data="<join_game_data>")]
pub fn join_game(join_game_data: Json<JoinGameData>) -> Value {
    // get player name
    let player_name = &join_game_data.name;

    // Join game
    // utils::join_room();

    // return json
    json!({"status": "ok"})
}