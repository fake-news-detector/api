extern crate diesel;

use diesel::prelude::*;
use diesel::update;
use data::schema::links::dsl;
use data::schema::links;
use scrapper::scrapper;

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "links"]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub content: Option<&'a str>,
}

pub fn find_or_create(url: &str, title: &str, conn: &PgConnection) -> QueryResult<Link> {
    let link = dsl::links.filter(dsl::url.eq(url)).first::<Link>(conn);

    link.or_else(|_| {
        let content = scrapper::extract_text(url).to_owned();

        let new_link: NewLink = NewLink {
            url: url,
            title: title,
            content: content.as_ref().map(String::as_ref),
        };
        diesel::insert(&new_link).into(dsl::links).get_result(conn)
    })
}

pub fn rescrape_content(link: &Link, conn: &PgConnection) -> QueryResult<Link> {
    let content = scrapper::extract_text(&link.url).to_owned();

    update(link).set(dsl::content.eq(content)).get_result(conn)
}
