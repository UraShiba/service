table! {
    chat_message (message_id) {
        message_id -> Int4,
        from_user_id -> Text,
        to_user_id -> Text,
        message_text -> Text,
        sent_datetime -> Text,
    }
}

table! {
    user_info (email) {
        user_id -> Text,
        user_name -> Text,
        email -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    chat_message,
    user_info,
);
