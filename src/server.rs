// #![feature(proc_macro_hygiene, decl_macro)]

// Uses
use rocket::http::Status;
use rocket_contrib::json::Json;
use crate::responses::{Move, Info,Movement };
use crate::requests::{Turn};

#[get("/")]
fn index() -> Json<Info> {
    Json(Info {
        apiversion: "1".to_string(),
        author: None,
        color: Some("#b7410e".to_string()),
        head: None,
        tail: None,
        version: Some("0".to_string()),
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
    Json(movement)
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, start, movement, end])
}

pub fn main() {
    rocket().launch();
}
