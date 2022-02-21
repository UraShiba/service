table! {
    user_info (user_id) {
        user_id -> Int4,
        user_name -> Text,
        login_name -> Text,
        pass -> Text,
    }
}

allow_tables_to_appear_in_same_query!(user_info,);
