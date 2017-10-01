#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::response::status;
use rocket::http::Status;
extern crate rocket;

#[get("/healthcheck")]
fn healthcheck() -> status::Custom<String> {
    status::Custom(Status::Ok, String::from("OK"))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![healthcheck, index])
        .launch();
}
