extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", {})
}
