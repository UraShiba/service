CREATE TABLE chat_message (
    message_id int NOT NULL primary key,
    from_user_id text NOT NULL references user_info(user_id),
    to_user_id text NOT NULL references user_info(user_id),
    message_text text NOT NULL,
    sent_date_time text NOT NULL
);