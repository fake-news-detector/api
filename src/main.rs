#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate select;

pub mod data;
pub mod endpoints;
pub mod lib;
pub mod scrapper;
pub mod jobs;

use rocket_contrib::Template;
use std::env;

fn main() {
    let arg1 = env::args().nth(1);
    match arg1 {
        Some(command) => jobs::run_job(&*command),
        _ => start_server(),
    }
}

fn start_server() {
    rocket::ignite()
        .manage(lib::pg_pool::init_pool())
        .mount(
            "/",
            routes![
                endpoints::healthcheck::healthcheck,
                endpoints::categories::get_categories,
                endpoints::votes::get_votes,
                endpoints::votes::post_vote,
                endpoints::links::get_all_links,
                endpoints::index::index,
            ],
        )
        .attach(Template::fairing())
        .launch();
}
