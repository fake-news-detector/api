extern crate diesel;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Json;
use diesel::prelude::*;
use commons::pg_pool::DbConn;
use commons::remote_ip::RemoteIp;
use data::hoax;
use commons::responders::*;


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
