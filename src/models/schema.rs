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
        project_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        title -> Text,
        details -> Nullable<Text>,
        uuid -> Text,
        complete -> Bool,
        inserted_date -> Nullable<Integer>,
    }
}

joinable!(todo -> project (project_id));

allow_tables_to_appear_in_same_query!(
    project,
    todo,
);
