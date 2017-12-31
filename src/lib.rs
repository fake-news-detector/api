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
pub mod commons;
pub mod scrapper;
pub mod jobs;

use rocket_contrib::Template;

pub fn run_job(command: &str) {
    jobs::run_job(command);
}

pub fn start_server() {
    rocket::ignite()
        .manage(commons::pg_pool::init_pool())
        .mount(
            "/",
            routes![
                endpoints::healthcheck::healthcheck,
                endpoints::categories::get_categories,
                endpoints::votes::get_votes,
                endpoints::votes::post_vote,
                endpoints::links::get_all_links,
                endpoints::index::index,
                endpoints::admin::admin
            ],
        )
        .attach(Template::fairing())
        .launch();
}
