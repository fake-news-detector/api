extern crate rocket;

use rocket::response::status;
use rocket::http::Status;

#[get("/healthcheck")]
fn healthcheck() -> status::Custom<String> {
    status::Custom(Status::Ok, String::from("OK"))
}
