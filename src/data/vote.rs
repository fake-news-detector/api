extern crate diesel;
extern crate reqwest;

use diesel::prelude::*;
use diesel::expression::dsl::*;
use data::schema::links::dsl;
use data::schema::votes;
use data::link::Link;
use diesel::types::{BigInt, Integer};
use data::verified_domains;
use scrapper::scrapper;

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(Link, Category)]
#[primary_key(link_id, uuid)]
pub struct Vote {
    pub link_id: i32,
    pub uuid: String,
    pub category_id: i32,
    pub ip: String,
    pub clickbait_title: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "votes"]
pub struct NewVote<'a> {
    pub link_id: i32,
    pub uuid: &'a str,
    pub category_id: i32,
    pub ip: &'a str,
    pub clickbait_title: Option<bool>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PeopleContentVote {
    pub category_id: i32,
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RobotContentVote {
    pub category_id: i32,
    pub chance: f32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PeopleTitleVote {
    pub clickbait: bool,
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RobotTitleVote {
    pub clickbait: bool,
    pub chance: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VerifiedVote {
    pub category_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ContentVotes {
    pub robot: Vec<RobotContentVote>,
    pub people: Vec<PeopleContentVote>,
}

#[derive(Serialize, Deserialize)]
pub struct TitleVotes {
    pub robot: RobotTitleVote,
    pub people: PeopleTitleVote,
}

#[derive(Serialize, Deserialize)]
pub struct AllVotes {
    domain: Option<VerifiedVote>,
    content: ContentVotes,
    title: TitleVotes,
    keywords: Vec<String>,
}

#[derive(Deserialize)]
pub struct RobinhoResponse {
    pub predictions: Vec<RobotContentVote>,
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

pub fn get_people_votes(url: &str, conn: &PgConnection) -> QueryResult<Vec<PeopleContentVote>> {
    let link: Result<Link, diesel::result::Error> = dsl::links.filter(dsl::url.eq(url)).first(conn);

    match link {
        Ok(link) => {
            let query = sql::<(Integer, BigInt)>(&format!(
                "SELECT category_id, count(*) FROM \
                 votes WHERE link_id = {} GROUP BY \
                 category_id",
                link.id
            ));
            query.load::<PeopleContentVote>(conn)
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub fn get_people_clickbait_votes(url: &str, conn: &PgConnection) -> QueryResult<PeopleTitleVote> {
    let link: Result<Link, diesel::result::Error> = dsl::links.filter(dsl::url.eq(url)).first(conn);

    match link {
        Ok(link) => {
            let query = sql::<(BigInt)>(&format!(
                "SELECT a.count - b.count FROM \
                 (SELECT count(*) AS count FROM votes WHERE link_id = {} AND clickbait_title = TRUE) AS a,
                 (SELECT count(*) AS count FROM votes WHERE link_id = {} AND clickbait_title = FALSE) AS b",
                link.id,
                link.id
            ));
            query.get_result::<i64>(conn).map(|count| {
                PeopleTitleVote {
                    clickbait: (count > 0),
                    count: count,
                }
            })
        }
        Err(_) => Ok(PeopleTitleVote {
            clickbait: false,
            count: 0,
        }),
    }
}

pub fn get_domain_category(url: &str) -> Option<VerifiedVote> {
    verified_domains::get_category(&url).map(|cid| VerifiedVote { category_id: cid })
}

pub fn get_all_votes(
    url: &str,
    title: &str,
    content: Option<&str>,
    conn: &PgConnection,
) -> QueryResult<AllVotes> {
    let domain = get_domain_category(&url);
    let content_ = match content {
        Some(text) => String::from(text),
        None => scrapper::extract_text(&url).unwrap_or(String::from("")),
    };
    let robinho_response = get_robinho_prediction(&title, &content_);
    let robinho_votes = robinho_response.predictions;
    let keywords = robinho_response.keywords;
    let people_votes = get_people_votes(&url, &*conn)?;
    let people_clickbait_votes = get_people_clickbait_votes(&url, &*conn)?;

    Ok(AllVotes {
        domain: domain,
        content: ContentVotes {
            robot: robinho_votes,
            people: people_votes,
        },
        title: TitleVotes {
            robot: RobotTitleVote {
                clickbait: false,
                chance: 0.0,
            },
            people: people_clickbait_votes,
        },
        keywords: keywords,
    })
}