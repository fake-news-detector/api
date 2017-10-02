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
pub struct Link {
    pub id: i32,
    pub title: String,
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
