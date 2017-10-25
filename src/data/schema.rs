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
        content -> Text,
    }
}

table! {
    votes (link_id, uuid) {
        link_id -> Integer,
        uuid -> Text,
        category_id -> Integer,
        ip -> Text,
    }
}
