use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

use crate::utils::{establish_connection, create_player};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct PlayerRequest {
    nickname: String,
    host: i32,
    score: i32,
    active: i32
}

#[post("/create", format = "json", data = "<data>")]
pub fn create_game_player(data: Json<PlayerRequest>) -> Value {

    use crate::schema::player::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    create_player(connection, data.nickname.clone(), data.host, data.score, data.active);


    let player_id = player.select(id).filter(nickname.eq(data.nickname.clone()))
        .or_filter(active.eq(1)).first::<Option<i32>>(connection).expect("Cannot load player id")
        .unwrap_or(0);

    json!({"status": "ok", "playerId": player_id})
}