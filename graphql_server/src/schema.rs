table! {
    chat_message (message_id) {
        message_id -> Int4,
        from_user_id -> Int4,
        to_user_id -> Int4,
        message_text -> Text,
        sent_datetime -> Text,
    }
}

table! {
    user_info (user_id) {
        user_id -> Int4,
        user_name -> Text,
        login_name -> Text,
        pass -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    chat_message,
    user_info,
);
