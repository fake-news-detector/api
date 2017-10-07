extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use data::schema::{categories, votes, links};

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
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
