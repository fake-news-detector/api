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
        content -> Nullable<Text>,
        verified_category_id -> Nullable<Integer>,
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
