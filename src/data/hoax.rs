extern crate diesel;

use diesel::prelude::*;
use data::schema::hoaxes;
use data::schema::hoaxes::dsl;

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "hoaxes"]
pub struct Hoax {
    pub id: i32,
    pub content: String,
    pub uuid: String,
    pub ip: String,
}

#[derive(Insertable)]
#[table_name = "hoaxes"]
pub struct NewHoax<'a> {
    pub content: &'a str,
    pub uuid: &'a str,
    pub ip: &'a str,
}

pub fn create(content: &str, uuid: &str, ip: &str, conn: &PgConnection) -> QueryResult<Hoax> {
    let new_hoax: NewHoax = NewHoax {
        content: content,
        uuid: uuid,
        ip: ip,
    };
    diesel::insert(&new_hoax).into(dsl::hoaxes).get_result(conn)
}