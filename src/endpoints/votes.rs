extern crate diesel;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate md5;

use rocket_contrib::Json;
use diesel::prelude::*;
use data::schema::votes::dsl::*;
use commons::pg_pool::DbConn;
use commons::remote_ip::RemoteIp;
use rocket::response::status;
use rocket::http::Status;
use diesel::result::Error::*;
use diesel::result::DatabaseErrorKind::*;
use data::vote::*;
use data::link::*;
use commons::responders::*;

#[derive(FromForm)]
pub struct GetVotesParams {
    url: String,
    title: String,
}

#[get("/votes?<params>")]
fn get_votes(params: GetVotesParams, conn: DbConn) -> QueryResult<Cached<Cors<Json<AllVotes>>>> {
    let all_votes = get_all_votes(&params.url, &params.title, None, &*conn)?;

    Ok(Cached(Cors(Json(all_votes))))
}

#[derive(FromForm)]
pub struct GetVotesByContentParams {
    content: String,
}

#[get("/votes_by_content?<params>")]
fn get_votes_by_content(
    params: GetVotesByContentParams,
    conn: DbConn,
) -> QueryResult<Cached<Cors<Json<AllVotes>>>> {
    let url = format!("{:x}", md5::compute(&params.content));

    let all_votes = get_all_votes(&url, &String::from(""), Some(&params.content), &*conn)?;

    Ok(Cached(Cors(Json(all_votes))))
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
) -> Cors<Result<Json<Vote>, status::Custom<String>>> {
    let link = find_or_create(&params.url, &params.title, &*conn);

    let insert_query = link.and_then(|l| {
        let new_vote: NewVote = NewVote {
            link_id: l.id,
            category_id: params.category_id,
            uuid: &params.uuid,
            ip: &remote_ip.ip,
        };
        diesel::insert(&new_vote).into(votes).get_result(&*conn)
    });

    let result = match insert_query {
        Ok(vote) => Ok(Json(vote)),
        Err(DatabaseError(UniqueViolation, _)) => Err(status::Custom(
            Status::BadRequest,
            String::from("Link already flagged by this user"),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            String::from("Internal Server Error"),
        )),
    };

    Cors(result)
}

#[options("/vote")]
fn post_vote_preflight() -> PreflightCors<()> {
    PreflightCors(())
}