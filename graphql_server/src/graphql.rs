use super::context::GraphQLContext;
use super::models::ChatMessage;
use super::models::UserInfo;
use super::schema::chat_message::dsl::chat_message;
use super::schema::user_info::dsl::user_info;
use chrono::{DateTime, Local};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{graphql_object, graphql_subscription, FieldError};
use juniper::{FieldResult, RootNode};
use std::pin::Pin;
use tokio_stream::Stream;

type UserInfoStream = Pin<Box<dyn Stream<Item = Result<UserInfo, FieldError>> + Send>>;
type ChatMessageStream =
    Pin<Box<dyn Stream<Item = Result<Option<ChatMessage>, FieldError>> + Send>>;
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

pub struct Query;
#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn getAccountByQuery(context: &GraphQLContext, id: i32) -> FieldResult<Vec<UserInfo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let results = user_info
            .find(id)
            .load::<UserInfo>(conn)
            .expect("Error loading customer");
        Ok(results)
    }
    async fn getMessageByQuery(context: &GraphQLContext, id: i32) -> FieldResult<Vec<ChatMessage>> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let results = chat_message
            .filter(crate::schema::chat_message::to_user_id.eq(id))
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
        user_id: i32,
        user_name: String,
        login_name: String,
        pass: String,
    ) -> FieldResult<UserInfo> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let user = UserInfo {
            user_id,
            user_name,
            login_name,
            pass,
        };
        diesel::insert_into(user_info)
            .values(&user)
            .execute(conn)
            .expect("Error saving new Users");
        context.sender.send(user.clone()).unwrap();
        Ok(user)
    }

    async fn send_message(
        context: &GraphQLContext,
        message_id: i32,
        from_user_id: i32,
        to_user_id: i32,
        message_text: String,
    ) -> FieldResult<ChatMessage> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let local_datetime: DateTime<Local> = Local::now();
        let sent_message = ChatMessage {
            message_id,
            from_user_id,
            to_user_id,
            message_text,
            sent_datetime: local_datetime.to_string(),
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
    async fn subscribe_account(context: &GraphQLContext) -> UserInfoStream {
        tokio_stream::wrappers::BroadcastStream::new(context.sender.subscribe())
            .map(|result| match result {
                Ok(userinfo) => Ok(userinfo),
                Err(err) => Err(FieldError::from(err.to_string())),
            })
            .boxed()
    }
    async fn subscribe_message(context: &GraphQLContext, id: i32) -> ChatMessageStream {
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
