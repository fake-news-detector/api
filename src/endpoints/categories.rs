extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;
use diesel::prelude::*;
use data::models::*;
use data::pg_pool::DbConn;
use data::schema::categories::dsl::*;

#[get("/categories")]
pub fn get_categories(connection: DbConn) -> QueryResult<Json<Vec<Category>>> {
    categories.load::<Category>(&*connection).map(
        |cats| Json(cats),
    )
}
