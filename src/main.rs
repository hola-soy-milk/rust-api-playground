#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{Json, Value};
use rocket::http::RawStr;

#[get("/<name>")]
fn name(name: &RawStr) -> Option<Json<Value>> {
    let response = format!("Hi there, {}!", name.as_str());
    Some(Json(json!({ "status": response })))
}

fn main() {
    rocket::ignite().mount("/", routes![name]).launch();
}
