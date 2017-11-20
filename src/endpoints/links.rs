extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::expression::dsl::*;
use commons::pg_pool::DbConn;
use diesel::types::{Nullable, Text, Integer, BigInt};

#[derive(Queryable, Serialize, Deserialize)]
struct LinkWithTopVote {
    id: i32,
    url: String,
    title: String,
    content: Option<String>,
    category_id: i32,
    count: i64,
}

#[get("/links/all")]
fn get_all_links(conn: DbConn) -> QueryResult<Json<Vec<LinkWithTopVote>>> {
    let query = sql::<(Integer, Text, Text, Nullable<Text>, Integer, BigInt)>(
        "SELECT links.id, links.url, links.title, links.content,
            top_votes.category_id, top_votes.total
         FROM links
         INNER JOIN
            (SELECT distinct on (link_id) category_id, link_id, count(category_id) total
            FROM votes
            GROUP by link_id, category_id
            ORDER by link_id, total DESC) AS top_votes
         ON top_votes.link_id = links.id
         ORDER BY links.id",
    );
    query.load::<LinkWithTopVote>(&*conn).map(Json)
}
