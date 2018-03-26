extern crate diesel;
extern crate md5;

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
    pub verified_category_id: Option<i32>,
    pub verified_clickbait_title: Option<bool>,
    pub removed: bool,
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub title: &'a str,
    pub content: Option<&'a str>,
}

pub fn find_or_create(
    url: &str,
    title: &str,
    content: Option<&str>,
    conn: &PgConnection,
) -> QueryResult<Link> {
    let link = dsl::links.filter(dsl::url.eq(url)).first::<Link>(conn);

    link.or_else(|_| {
        let content_ = match content {
            Some(text) => Some(String::from(text)),
            None => scrapper::extract_text(url).to_owned(),
        };

        let new_link: NewLink = NewLink {
            url: url,
            title: title,
            content: content_.as_ref().map(String::as_ref),
        };
        diesel::insert(&new_link).into(dsl::links).get_result(conn)
    })
}

pub fn rescrape_content(link: &Link, conn: &PgConnection) -> QueryResult<Link> {
    let content = scrapper::extract_text(&link.url).to_owned();

    update(link).set(dsl::content.eq(content)).get_result(conn)
}

pub fn set_verified_category_id(
    id: i32,
    category_id: Option<i32>,
    conn: &PgConnection,
) -> QueryResult<Link> {
    let link = dsl::links.filter(dsl::id.eq(id)).first::<Link>(conn)?;

    update(&link)
        .set(dsl::verified_category_id.eq(category_id))
        .get_result(conn)
}

pub fn hash_from_content(content: &str) -> String {
    format!("{:x}", md5::compute(&content))
}

pub fn set_removed(id: i32, removed: bool, conn: &PgConnection) -> QueryResult<Link> {
    let link = dsl::links.filter(dsl::id.eq(id)).first::<Link>(conn)?;

    update(&link).set(dsl::removed.eq(removed)).get_result(conn)
}

pub fn set_verified_clickbait_title(
    id: i32,
    clickbait_title: Option<bool>,
    conn: &PgConnection,
) -> QueryResult<Link> {
    let link = dsl::links.filter(dsl::id.eq(id)).first::<Link>(conn)?;

    update(&link)
        .set(dsl::verified_clickbait_title.eq(clickbait_title))
        .get_result(conn)
}