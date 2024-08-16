// @generated automatically by Diesel CLI.

diesel::table! {
    conversation (id) {
        id -> Int4,
        user1_id -> Int4,
        user2_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    message (id) {
        id -> Int4,
        conversation_id -> Int4,
        sender_id -> Nullable<Int4>,
        content -> Text,
        timestamp -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 200]
        username -> Varchar,
        #[max_length = 200]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(message -> conversation (conversation_id));
diesel::joinable!(message -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversation,
    message,
    users,
);
