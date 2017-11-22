#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::content;

#[get("/")]
fn json() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

fn main() {
    rocket::ignite().mount("/", routes![json]).launch();
}
