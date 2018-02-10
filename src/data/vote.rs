extern crate diesel;
extern crate reqwest;

use diesel::prelude::*;
use diesel::expression::dsl::*;
use data::schema::links::dsl;
use data::schema::votes;
use data::link::Link;
use diesel::types::{BigInt, Integer};
use data::verified_list;
use scrapper::scrapper;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(Link, Category)]
#[primary_key(link_id, uuid)]
pub struct Vote {
    pub link_id: i32,
    pub uuid: String,
    pub category_id: i32,
    pub ip: String,
}

#[derive(Insertable)]
#[table_name = "votes"]
pub struct NewVote<'a> {
    pub link_id: i32,
    pub uuid: &'a str,
    pub category_id: i32,
    pub ip: &'a str,
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
pub struct AllVotes {
    verified: Option<VerifiedVote>,
    robot: Vec<RobotVote>,
    people: Vec<PeopleVote>,
    keywords: Vec<String>,
}

#[derive(Deserialize)]
pub struct RobinhoResponse {
    pub predictions: Vec<RobotVote>,
    pub keywords: Vec<String>,
}

pub fn get_robinho_prediction(title: &str, content: &str) -> RobinhoResponse {
    let mut prediction_url = reqwest::Url::parse("https://robinho.fakenewsdetector.org/predict")
        .unwrap();
    prediction_url.query_pairs_mut().append_pair("title", title);
    prediction_url.query_pairs_mut().append_pair(
        "content",
        content,
    );

    reqwest::get(prediction_url)
        .and_then(|mut r| r.json())
        .unwrap_or(RobinhoResponse {
            predictions: Vec::new(),
            keywords: Vec::new(),
        })
}

pub fn get_people_votes(url: &str, conn: &PgConnection) -> QueryResult<Vec<PeopleVote>> {
    let link: Result<Link, diesel::result::Error> = dsl::links.filter(dsl::url.eq(url)).first(conn);

    match link {
        Ok(link) => {
            let query = sql::<(Integer, BigInt)>(&format!(
                "SELECT category_id, count(*) FROM \
                 votes WHERE link_id = {} GROUP BY \
                 category_id",
                link.id
            ));
            query.load::<PeopleVote>(conn)
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub fn get_verified_category(url: &str) -> Option<VerifiedVote> {
    verified_list::get_category(&url).map(|cid| VerifiedVote { category_id: cid })
}

pub fn get_all_votes(
    url: &str,
    title: &str,
    content: Option<&str>,
    conn: &PgConnection,
) -> QueryResult<AllVotes> {
    let mut robinho_votes = vec![];
    let mut people_votes = vec![];
    let mut keywords = vec![];

    let verified = get_verified_category(&url);
    if verified.is_none() {
        let content_ = match content {
            Some(text) => String::from(text),
            None => scrapper::extract_text(&url).unwrap_or(String::from("")),
        };

        let robinho_response = get_robinho_prediction(&title, &content_);
        robinho_votes = robinho_response.predictions;
        keywords = robinho_response.keywords;
        people_votes = get_people_votes(&url, &*conn)?;
    }

    Ok(AllVotes {
        verified: verified,
        robot: robinho_votes,
        people: people_votes,
        keywords: keywords,
    })
}