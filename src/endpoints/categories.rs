extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;
use diesel::prelude::*;
use data::models::*;
use lib::pg_pool::DbConn;
use data::schema::categories::dsl::*;

#[get("/categories")]
pub fn get_categories(conn: DbConn) -> QueryResult<Json<Vec<Category>>> {
    categories.load::<Category>(&*conn).map(Json)
}
