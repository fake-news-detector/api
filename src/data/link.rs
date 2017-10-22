extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
use diesel::prelude::*;
use data::schema::links::dsl::*;
use data::models::*;

pub fn find_or_create(url_: &String, title_: &String, conn: &PgConnection) -> QueryResult<Link> {
    let mut link = links.filter(url.eq(url_)).first::<Link>(conn);
    if link.is_err() {
        let new_link: NewLink = NewLink {
            url: url_,
            title: title_,
        };
        link = diesel::insert(&new_link).into(links).get_result(conn)
    };

    link
}
