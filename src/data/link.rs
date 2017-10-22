extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
use diesel::prelude::*;
use data::schema::links::dsl;
use data::schema::links;

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

pub fn find_or_create(url_: &String, title_: &String, conn: &PgConnection) -> QueryResult<Link> {
    let mut link = dsl::links.filter(dsl::url.eq(url_)).first::<Link>(conn);
    if link.is_err() {
        let new_link: NewLink = NewLink {
            url: url_,
            title: title_,
        };
        link = diesel::insert(&new_link).into(dsl::links).get_result(conn)
    };

    link
}
