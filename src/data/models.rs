extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use data::schema::{categories, votes, links};

#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable)]
#[table_name = "links"]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub title: &'a str,
}

#[derive(Serialize, Deserialize)]
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Link, Category)]
#[primary_key(link_id, category_id, uuid)]
pub struct Vote {
    pub link_id: i32,
    pub category_id: i32,
    pub uuid: String,
    pub ip: String,
}

#[derive(Insertable)]
#[table_name = "votes"]
pub struct NewVote<'a> {
    pub link_id: i32,
    pub category_id: i32,
    pub uuid: &'a str,
    pub ip: &'a str,
}
