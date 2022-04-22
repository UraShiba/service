CREATE TABLE user_info(
  user_id text NOT NULL,
  user_name text NOT NULL,
  email text NOT NULL,
  password text NOT NULL,
  CONSTRAINT pk_user PRIMARY KEY (user_id)
);

CREATE TABLE chat_message
(
message_id int NOT NULL primary key,
from_user_id text NOT NULL references user_info(user_id),
to_user_id text NOT NULL references user_info(user_id),
message_text NOT NULL text,
sent_datetime NOT NULL text
);