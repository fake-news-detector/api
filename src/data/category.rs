extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
use data::schema::categories;

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}
