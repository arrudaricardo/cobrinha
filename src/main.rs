#![feature(proc_macro_hygiene, decl_macro)]

mod requests;
mod responses;
use requests::Turn;
use responses::{Info, Move, Movement};

// External crates
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> Json<Info> {
    Json(Info {
        apiversion: "1".to_string(),
        author: Some("Ricardo e Arruda".to_string()),
        color: Some("#f8ff38".to_string()),
        head: Some("gamer".to_string()),
        tail: Some("pixel".to_string()),
        version: Some("rust".to_string()),
    })
}

#[post("/start")]
fn start() -> Status {
    Status::Ok
}

#[post("/move", data = "<req>")]
fn movement(req: Json<Turn>) -> Json<Move> {
    let movement = Move::new(Movement::Right);
    // Logic goes here
    println!("{:?}", req.game);
    
    Json(movement)
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, start, movement, end])
}

fn main() {
    rocket().launch();
}
