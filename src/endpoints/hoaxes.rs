extern crate diesel;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::expression::dsl::*;
use commons::pg_pool::DbConn;
use commons::remote_ip::RemoteIp;
use data::hoax;
use commons::responders::*;
use diesel::types::{Text, Integer};



#[derive(Deserialize)]
struct PostHoax {
    uuid: String,
    content: String,
}

#[post("/hoax", data = "<params>")]
fn post_hoax(
    params: Json<PostHoax>,
    conn: DbConn,
    remote_ip: RemoteIp,
) -> QueryResult<Cors<Json<hoax::Hoax>>> {
    hoax::create(&params.content, &params.uuid, &remote_ip.ip, &*conn)
        .map(Json)
        .map(Cors)
}

#[options("/hoax")]
fn post_hoax_preflight() -> PreflightCors<()> {
    PreflightCors(())
}

#[derive(Queryable, Serialize, Deserialize)]
struct HoaxData {
    id: i32,
    content: String,
}

#[get("/hoaxes/all")]
fn get_all_hoaxes(conn: DbConn) -> QueryResult<Json<Vec<HoaxData>>> {
    let query = sql::<(Integer, Text)>(
        "SELECT hoaxes.id, hoaxes.content
         FROM hoaxes
         ORDER BY hoaxes.id DESC",
    );
    query.load::<HoaxData>(&*conn).map(Json)
}