extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use data::schema::links::dsl::*;
use data::models::*;
use data::pg_pool::DbConn;

#[derive(FromForm)]
struct GetVotesParams {
    link: String,
}

#[get("/votes?<params>")]
fn get_votes(params: GetVotesParams, connection: DbConn) -> QueryResult<Json<Vec<Vote>>> {
    let link: Result<Link, diesel::result::Error> = links.first(&*connection);
    match link {
        Ok(link) => {
            let votes_ = Vote::belonging_to(&link).load(&*connection);
            votes_.map(Json)
        }
        Err(_) => Ok(Json(Vec::new())),
    }
}
