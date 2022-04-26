use crate::auth::{authorize, encode, hashing_password, JWT_SECRET};
use crate::context::GraphQLContext;
use crate::models::schema::{chat_message::dsl::chat_message, user_info::dsl::user_info};
use crate::models::{ChatMessage, Response, UserInfo};

use chrono::{DateTime, Local};
use diesel::{pg::PgConnection, prelude::*};
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult, RootNode};
use pwhash::bcrypt;
use std::pin::Pin;
use tokio_stream::Stream;
use uuid::Uuid;

type ChatMessageStream =
    Pin<Box<dyn Stream<Item = Result<Option<ChatMessage>, FieldError>> + Send>>;
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

pub struct Query;
#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn getAccounts(context: &GraphQLContext, token: String) -> FieldResult<Vec<UserInfo>> {
        match authorize(&JWT_SECRET.to_string(), token) {
            Ok(_) => {
                let conn: &PgConnection = &context.pool.get().unwrap();
                let results = user_info
                    .load::<UserInfo>(conn)
                    .expect("Error loading customer");
                Ok(results)
            }
            Err(e) => Err(FieldError::from(e.message)),
        }
    }
    async fn getMessageByQuery(
        context: &GraphQLContext,
        id: String,
    ) -> FieldResult<Vec<ChatMessage>> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let results = chat_message
            .filter(crate::models::schema::chat_message::to_user_id.eq(id))
            .load::<ChatMessage>(conn)
            .expect("Error loading customer");
        Ok(results)
    }
}

pub struct Mutation;
#[graphql_object(context = GraphQLContext)]
impl Mutation {
    async fn signUp(
        context: &GraphQLContext,
        user_name: String,
        email: String,
        pass: String,
    ) -> FieldResult<Response> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let user_id = Uuid::new_v4().to_hyphenated().to_string();
        let hashing_pass = hashing_password(&pass);

        let results = user_info
            .filter(crate::models::schema::user_info::email.eq(email.clone()))
            .limit(1)
            .load::<UserInfo>(conn);

        let user = UserInfo {
            user_id,
            user_name,
            email,
            pass: hashing_pass,
        };

        match results {
            Ok(users) => {
                if users.len() > 0 {
                    Err(FieldError::from("Email already exists".to_string()))
                } else {
                    diesel::insert_into(user_info)
                        .values(&user)
                        .execute(conn)
                        .expect("Error adding new Users");

                    let token = encode(&JWT_SECRET.to_string(), &user.user_id);
                    Ok(Response { token })
                }
            }
            Err(_) => Err(FieldError::from("Unexpected error".to_string())),
        }
    }

    async fn signIn(
        context: &GraphQLContext,
        email: String,
        pass: String,
    ) -> FieldResult<Response> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let results = user_info
            .filter(crate::models::schema::user_info::email.eq(email.clone()))
            .limit(1)
            .load::<UserInfo>(conn);

        match results {
            Ok(users) => {
                if users.len() > 0 {
                    let user = users.first().unwrap();
                    match bcrypt::verify(pass, &user.pass) {
                        true => {
                            let token = encode(&JWT_SECRET.to_string(), &user.user_id);
                            Ok(Response { token })
                        }
                        false => Err(FieldError::from("Password is wrong".to_string())),
                    }
                } else {
                    Err(FieldError::from("Email doesn't exist".to_string()))
                }
            }
            Err(e) => Err(FieldError::from(e)),
        }
    }

    async fn send_message(
        context: &GraphQLContext,
        message_id: i32,
        from_user_id: String,
        to_user_id: String,
        message_text: String,
    ) -> FieldResult<ChatMessage> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let local_date_time: DateTime<Local> = Local::now();
        let sent_message = ChatMessage {
            message_id,
            from_user_id,
            to_user_id,
            message_text,
            sent_date_time: local_date_time.to_string(),
        };
        diesel::insert_into(chat_message)
            .values(&sent_message)
            .execute(conn)
            .expect("Error sending a chat message");
        context
            .chat_message_sender
            .send(sent_message.clone())
            .unwrap();
        Ok(sent_message)
    }
}

pub struct Subscription;
#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn subscribe_message(context: &GraphQLContext, id: String) -> ChatMessageStream {
        tokio_stream::wrappers::BroadcastStream::new(context.chat_message_sender.subscribe())
            .map(move |result| match result {
                Ok(sent_message) => {
                    let result_data;
                    let data = sent_message.clone();
                    if data.to_user_id == id {
                        result_data = Some(sent_message);
                    } else if data.from_user_id == id {
                        result_data = Some(sent_message);
                    } else {
                        result_data = None;
                    }
                    Ok(result_data)
                }
                Err(err) => Err(FieldError::from(err.to_string())),
            })
            .boxed()
    }
}
