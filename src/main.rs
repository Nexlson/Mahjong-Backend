#[macro_use]extern crate rocket;

use rocket::Request;

mod routes;
mod tests;
mod utils;
mod schema;
mod models;

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
        .mount("/game", routes![routes::game::next_round])
        .mount("/room", routes![routes::room::create_game, routes::room::end_game, routes::room::join_game, 
            routes::room::start_game])
        .register("/", catchers![not_found_page, unprocessable_entity])
}