extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
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
use data::verified_list;

mod types {
    #[derive(FromForm)]
    pub struct GetVotesParams {
        pub url: String,
        pub title: String,
    }
    #[derive(Queryable, Serialize, Deserialize)]
    pub struct PeopleVote {
        pub category_id: i32,
        pub count: i64,
    }
    #[derive(Serialize, Deserialize)]
    pub struct RobotVote {
        pub category_id: i32,
        pub chance: f32,
    }
    #[derive(Serialize, Deserialize)]
    pub struct VerifiedVote {
        pub category_id: i32,
    }
    #[derive(Serialize, Deserialize)]
    pub struct GetVotesResponse {
        pub verified: Option<VerifiedVote>,
        pub robot: Vec<RobotVote>,
        pub people: Vec<PeopleVote>,
    }
    #[derive(Deserialize)]
    pub struct RobinhoResponse {
        pub predictions: Vec<RobotVote>,
    }
}
use self::types::*;

#[get("/votes?<params>")]
fn get_votes(params: GetVotesParams, conn: DbConn) -> QueryResult<Json<GetVotesResponse>> {
    let mut prediction_url = reqwest::Url::parse("https://robinho.herokuapp.com/predict").unwrap();
    prediction_url.query_pairs_mut().append_pair(
        "title",
        &params.title,
    );

    let robinho_votes: RobinhoResponse =
        reqwest::get(prediction_url)
            .and_then(|mut r| r.json())
            .unwrap_or(RobinhoResponse { predictions: Vec::new() });

    let link: Result<Link, diesel::result::Error> = links.filter(url.eq(&params.url)).first(&*conn);
    let votes_count = match link {
        Ok(link) => {
            let query = sql::<(Integer, BigInt)>(&format!(
                "SELECT category_id, count(*) FROM votes WHERE link_id = {} GROUP BY category_id",
                link.id
            ));
            query.load::<PeopleVote>(&*conn)
        }
        Err(_) => Ok(Vec::new()),
    };

    let verified_vote =
        verified_list::get_category(params.url).map(|cid| VerifiedVote { category_id: cid });

    match votes_count {
        Ok(people_votes) => {
            Ok(Json(GetVotesResponse {
                verified: verified_vote,
                robot: robinho_votes.predictions,
                people: people_votes,
            }))
        }
        Err(err) => Err(err),
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
