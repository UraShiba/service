CREATE TABLE user_info(
  user_id text NOT NULL,
  user_name text NOT NULL,
  email text NOT NULL,
  pass text NOT NULL,
  CONSTRAINT pk_user PRIMARY KEY (user_id)
);