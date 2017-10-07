extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;
use diesel::prelude::*;
use diesel::expression::dsl::*;
use data::schema::links::dsl::*;
use data::schema::votes::dsl::*;
use data::models::*;
use data::pg_pool::DbConn;
use diesel::types::{Integer, BigInt};
use lib::remote_ip::RemoteIp;
use rocket::response::status;
use rocket::http::Status;
use diesel::result::Error::*;
use diesel::result::DatabaseErrorKind::*;

mod types {
    #[derive(FromForm)]
    pub struct GetVotesParams {
        pub url: String,
    }
    #[derive(Queryable, Serialize, Deserialize)]
    pub struct VotesCount {
        pub category_id: i32,
        pub count: i64,
    }
}
use self::types::*;

#[get("/votes?<params>")]
fn get_votes(params: GetVotesParams, conn: DbConn) -> QueryResult<Json<Vec<VotesCount>>> {
    let link: Result<Link, diesel::result::Error> = links.filter(url.eq(&params.url)).first(&*conn);
    match link {
        Ok(link) => {
            let query = sql::<(Integer, BigInt)>(&format!(
                "SELECT category_id, count(*) FROM votes WHERE link_id = {} GROUP BY category_id",
                link.id
            ));
            query.load::<VotesCount>(&*conn).map(Json)
        }
        Err(_) => Ok(Json(Vec::new())),
    }
}

#[derive(Deserialize)]
struct PostVote {
    uuid: String,
    url: String,
    title: String,
    category_id: i32,
}

#[post("/vote", data = "<params>")]
fn post_vote(
    params: Json<PostVote>,
    conn: DbConn,
    remote_ip: RemoteIp,
) -> Result<Json<Vote>, status::Custom<String>> {
    let mut link = links.filter(url.eq(&params.url)).first::<Link>(&*conn);
    if link.is_err() {
        let new_link: NewLink = NewLink {
            url: &params.url,
            title: &params.title,
        };
        link = diesel::insert(&new_link).into(links).get_result(&*conn)
    };
    let insert_query = link.and_then(|l| {
        let new_vote: NewVote = NewVote {
            link_id: l.id,
            category_id: params.category_id,
            uuid: &params.uuid,
            ip: &remote_ip.ip,
        };
        diesel::insert(&new_vote).into(votes).get_result(&*conn)
    }).map(Json);

    match insert_query {
        Ok(vote) => Ok(vote),
        Err(DatabaseError(UniqueViolation, _)) => Err(status::Custom(
            Status::BadRequest,
            String::from("Link already flagged by this user"),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            String::from("Internal Server Error"),
        )),
    }
}
