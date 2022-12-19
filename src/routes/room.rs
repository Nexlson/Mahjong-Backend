use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

use crate::utils::{establish_connection, create_room, join_room};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CreateRoomData {
    host: i32,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct JoinRoomData {
    player_id: i32,
    room_id: i32
}

// Create a game
#[post("/create", format = "json", data = "<data>")]
pub fn create_game_room(data: Json<CreateRoomData>) -> Value {
    use crate::schema::room::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    create_room(connection, data.host);

    let room_id = room.select(id).filter(player_one.eq(data.host))
        .first::<Option<i32>>(connection).expect("Cannot load room id").unwrap_or(0);

    // return json
    json!({"status": "ok", "roomId": room_id})
}

// Join a room
#[post("/join", format = "json", data = "<data>")]
pub fn join_game_room(data: Json<JoinRoomData>) -> Value {

    let connection = &mut establish_connection();

    let result = join_room(connection, data.room_id, data.player_id);

    // return json
    if result == 0 {
        json!({"status": "error", "roomId": data.room_id})

    }else {
        json!({"status": "ok", "roomId": data.room_id})
    }
    
}
