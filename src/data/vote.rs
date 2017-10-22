extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
use diesel::prelude::*;
use diesel::expression::dsl::*;
use data::schema::links::dsl as linksdsl;
use data::models::*;
use lib::pg_pool::DbConn;
use diesel::types::{Integer, BigInt};
use data::verified_list;

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
#[derive(Deserialize)]
pub struct RobinhoResponse {
    pub predictions: Vec<RobotVote>,
}

pub fn get_robinho_prediction(title: &String) -> RobinhoResponse {
    let mut prediction_url = reqwest::Url::parse("https://robinho.herokuapp.com/predict").unwrap();
    prediction_url.query_pairs_mut().append_pair("title", title);

    reqwest::get(prediction_url)
        .and_then(|mut r| r.json())
        .unwrap_or(RobinhoResponse { predictions: Vec::new() })
}

pub fn get_people_votes(url: &String, conn: DbConn) -> QueryResult<Vec<PeopleVote>> {
    let link: Result<Link, diesel::result::Error> =
        linksdsl::links.filter(linksdsl::url.eq(url)).first(&*conn);

    match link {
        Ok(link) => {
            let query = sql::<(Integer, BigInt)>(&format!(
                "SELECT category_id, count(*) FROM votes WHERE link_id = {} GROUP BY category_id",
                link.id
            ));
            query.load::<PeopleVote>(&*conn)
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub fn get_verified_category(url: String) -> Option<VerifiedVote> {
    verified_list::get_category(url).map(|cid| VerifiedVote { category_id: cid })
}
