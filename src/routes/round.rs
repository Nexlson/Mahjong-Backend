use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

use crate::utils::{establish_connection, create_round};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CreateRoundData {
    room_id: i32,
    active: i32
}

// Create a round
#[post("/create", format = "json", data = "<data>")]
pub fn create_game_round(data: Json<CreateRoundData>) -> Value {
    use crate::schema::round::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    create_round(connection, data.room_id, data.active);

    let round_id = round.select(id).filter(room_id.eq(data.room_id)).or_filter(active.eq(1))
        .first::<Option<i32>>(connection).expect("Cannot load round id").unwrap_or(0);

    // return json
    json!({"status": "ok", "roomId": round_id})
}