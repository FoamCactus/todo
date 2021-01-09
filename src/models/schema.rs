table! {
    project (id) {
        id -> Integer,
        title -> Text,
        uuid -> Text,
    }
}

table! {
    todo (id) {
        id -> Integer,
        project_id -> Integer,
        title -> Text,
        details -> Nullable<Text>,
        uuid -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    project,
    todo,
);
