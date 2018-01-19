extern crate diesel;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;

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
use scrapper::scrapper;
use commons::responders::*;

#[derive(FromForm)]
pub struct GetVotesParams {
    url: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
pub struct GetVotesResponse {
    verified: Option<VerifiedVote>,
    robot: Vec<RobotVote>,
    people: Vec<PeopleVote>,
}

#[get("/votes?<params>")]
fn get_votes(
    params: GetVotesParams,
    conn: DbConn,
) -> QueryResult<Cached<Cors<Json<GetVotesResponse>>>> {
    let mut robinho_votes = vec![];
    let mut people_votes = vec![];

    let verified = get_verified_category(&params.url);
    if verified.is_none() {
        let content = scrapper::extract_text(&params.url).unwrap_or(String::from(""));
        robinho_votes = get_robinho_prediction(&params.title, &content).predictions;
        people_votes = get_people_votes(&params.url, &*conn)?;
    }

    Ok(Cached(Cors(Json(GetVotesResponse {
        verified: verified,
        robot: robinho_votes,
        people: people_votes,
    }))))
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