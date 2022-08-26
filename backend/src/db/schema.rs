table! {
    items (id) {
        id -> Integer,
        path -> Text,
    }
}

table! {
    items_tags (item_id, tag_id) {
        item_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(items_tags -> items (item_id));
joinable!(items_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    items,
    items_tags,
    tags,
);
