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
    let url = &hash_from_content(&params.content);

    let all_votes = get_all_votes(url, &String::from(""), Some(&params.content), &*conn)?;

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
    post_vote_(
        &params.url,
        &params.title,
        None,
        &params.uuid,
        params.category_id,
        conn,
        remote_ip,
    )
}

#[options("/vote")]
fn post_vote_preflight() -> PreflightCors<()> {
    PreflightCors(())
}

#[derive(Deserialize)]
struct PostVoteByContent {
    uuid: String,
    content: String,
    category_id: i32,
}

#[post("/vote_by_content", data = "<params>")]
fn post_vote_by_content(
    params: Json<PostVoteByContent>,
    conn: DbConn,
    remote_ip: RemoteIp,
) -> Cors<Result<Json<Vote>, status::Custom<String>>> {
    let url = &hash_from_content(&params.content);

    post_vote_(
        url,
        &String::from(""),
        Some(&params.content),
        &params.uuid,
        params.category_id,
        conn,
        remote_ip,
    )
}

#[options("/vote_by_content")]
fn post_vote_by_content_preflight() -> PreflightCors<()> {
    PreflightCors(())
}

fn post_vote_(
    url: &str,
    title: &str,
    content: Option<&str>,
    uuid_: &str,
    category_id_: i32,
    conn: DbConn,
    remote_ip: RemoteIp,
) -> Cors<Result<Json<Vote>, status::Custom<String>>> {
    let link = find_or_create(&url, &title, content, &*conn);

    let insert_query = link.and_then(|l| {
        let new_vote: NewVote = NewVote {
            link_id: l.id,
            category_id: category_id_,
            uuid: &uuid_,
            ip: &remote_ip.ip,
        };
        diesel::insert(&new_vote).into(votes).get_result(&*conn)
    });

    let result = check_already_flagged_error(insert_query).map(Json);

    Cors(result)
}

fn check_already_flagged_error(query: QueryResult<Vote>) -> Result<Vote, status::Custom<String>> {
    match query {
        Ok(vote) => Ok(vote),
        Err(DatabaseError(UniqueViolation, _)) => Err(status::Custom(
            Status::BadRequest,
            String::from(
                "Content already flagged by this user",
            ),
        )),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            String::from("Internal Server Error"),
        )),
    }
}