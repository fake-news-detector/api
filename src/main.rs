#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::Json;
mod pg_pool;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
pub mod models;
pub mod schema;
use self::models::*;
use self::diesel::prelude::*;
use self::diesel::BelongingToDsl;
use schema::categories::dsl::*;
use schema::votes::dsl::*;
use schema::links::dsl::*;
#[macro_use]
extern crate serde_derive;

#[get("/healthcheck")]
fn healthcheck() -> status::Custom<String> {
    status::Custom(Status::Ok, String::from("OK"))
}


#[get("/categories")]
fn get_categories(connection: pg_pool::DbConn) -> QueryResult<Json<Vec<Category>>> {
    categories.load::<Category>(&*connection).map(
        |cats| Json(cats),
    )
}

#[derive(FromForm)]
struct GetVotesParams {
    link: String,
}

#[get("/votes?<params>")]
fn get_votes(params: GetVotesParams, connection: pg_pool::DbConn) -> QueryResult<Json<Vec<Vote>>> {
    let link: Result<Link, diesel::result::Error> = links.first(&*connection);
    match link {
        Ok(link) => {
            let votes_ = Vote::belonging_to(&link).load(&*connection);
            votes_.map(Json)
        }
        Err(_) => Ok(Json(Vec::new())),
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .manage(pg_pool::init_pool())
        .mount("/", routes![healthcheck, get_categories, get_votes, index])
        .launch();
}
