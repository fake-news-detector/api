#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate fake_news_api;

use rocket::http::Status;
use rocket::local::Client;
use rocket_contrib::Template;
use fake_news_api::endpoints::index;

const TEMPLATE_ROOT: &'static str = "src/views";

#[test]
fn return_index_page() {
    let client = create_index_client();

    let mut response = client.get("/").dispatch();

    let expected = get_template_content("index");
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), expected);
}

fn create_index_client() -> Client {
    let rocket = rocket::ignite()
        .mount("/", routes![index::index])
        .attach(Template::fairing());

    Client::new(rocket).unwrap()
}

fn get_template_content<S>(template_name: S) -> Option<String>
    where S: Into<String> {
        
    let content = Template::show(TEMPLATE_ROOT, template_name.into(), {}).unwrap();

    Some(content)
}
