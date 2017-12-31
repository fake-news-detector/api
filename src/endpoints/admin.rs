extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Template;

#[get("/admin")]
fn admin() -> Template {
    Template::render("admin", {})
}
