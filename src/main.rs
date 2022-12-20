#[macro_use]extern crate rocket;

use rocket::Request;

mod routes;
mod utils;
mod models;
mod schema;

// index page
#[get("/")]
fn index() -> &'static str {
    "Welcome to mahjong game service!"
}

// catch error page
#[catch(404)]
fn not_found_page(req: &Request) -> String{
    format!("The path {} is not found", req.uri())
}

#[catch(422)]
fn unprocessable_entity() -> String {
    format!("Cannot process, check your input again!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/round", routes![routes::round::create_game_round, routes::round::get_game_summary])
        .mount("/room", routes![routes::room::create_game_room, routes::room::join_game_room])
        .mount("/player", routes![routes::player::create_game_player])
        .register("/", catchers![not_found_page, unprocessable_entity])
}