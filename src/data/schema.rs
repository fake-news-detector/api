extern crate diesel;

table! {
    categories {
        id -> Integer,
        name -> Text,
    }
}

table! {
    links {
        id -> Integer,
        url -> Text,
        title -> Text,
    }
}

table! {
    votes (link_id, category_id, uuid) {
        link_id -> Integer,
        category_id -> Integer,
        uuid -> Text,
        ip -> Text,
    }
}
