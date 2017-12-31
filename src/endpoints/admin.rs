extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;
use rocket_contrib::Template;

#[get("/admin")]
fn admin() -> Template {
    Template::render("admin", {})
}

#[derive(Deserialize)]
struct LoginParams {
    email: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    email: String,
}

#[post("/admin/login", data = "<params>")]
fn login(params: Json<LoginParams>) -> Json<LoginResponse> {
    Json(LoginResponse { email: params.email.to_owned() })
}
