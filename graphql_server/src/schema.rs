table! {
    chat_message (message_id) {
        message_id -> Int4,
        from_user_id -> Text,
        to_user_id -> Text,
        message_text -> Text,
        sent_date_time -> Text,
    }
}

table! {
    user_info (user_id) {
        user_id -> Text,
        user_name -> Text,
        email -> Text,
        pass -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    chat_message,
    user_info,
);
