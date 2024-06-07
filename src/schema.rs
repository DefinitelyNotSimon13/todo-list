// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Int4,
        uuid -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        completed -> Bool,
        deadline -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
