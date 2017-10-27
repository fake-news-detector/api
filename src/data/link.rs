extern crate diesel;

use diesel::prelude::*;
use data::schema::links::dsl;
use data::schema::links;
use scrapper::scrapper;

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "links"]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub content: &'a str,
}

pub fn find_or_create(url: &String, title: &String, conn: &PgConnection) -> QueryResult<Link> {
    let link = dsl::links.filter(dsl::url.eq(url)).first::<Link>(conn);

    link.or_else(|_| {
        let content = scrapper::extract_text(url).unwrap_or(String::from(""));
        let new_link: NewLink = NewLink {
            url: url,
            title: title,
            content: &content,
        };
        diesel::insert(&new_link).into(dsl::links).get_result(conn)
    })
}
