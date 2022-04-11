use super::context::GraphQLContext;
use super::models::ChatMessage;
use super::models::UserInfo;
use super::schema::user_info::dsl::user_info;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{graphql_object, graphql_subscription, FieldError};
use juniper::{FieldResult, RootNode};
use std::pin::Pin;
use tokio_stream::Stream;

type UserInfoStream = Pin<Box<dyn Stream<Item = Result<UserInfo, FieldError>> + Send>>;
type ChatMessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, FieldError>> + Send>>;
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
        sent_datetime: String,
    ) -> FieldResult<ChatMessage> {
        let chat_message = ChatMessage {
            message_id,
            from_user_id,
            to_user_id,
            message_text,
            sent_datetime,
        };
        context
            .chat_message_sender
            .send(chat_message.clone())
            .unwrap();
        Ok(chat_message)
    }
}

pub struct Subscription;
#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn subscribe_account(context: &GraphQLContext) -> UserInfoStream {
        let conn: &PgConnection = &context.pool.get().unwrap();
        tokio_stream::wrappers::BroadcastStream::new(context.sender.subscribe())
            .map(|result| match result {
                Ok(userinfo) => Ok(userinfo),
                Err(err) => Err(FieldError::from(err.to_string())),
            })
            .boxed()
    }
    async fn subscribe_message(context: &GraphQLContext, self_id: i32) -> ChatMessageStream {
        tokio_stream::wrappers::BroadcastStream::new(context.chat_message_sender.subscribe())
            .map(|result| match result {
                Ok(chat_message) => Ok(chat_message),
                Err(err) => Err(FieldError::from(err.to_string())),
            })
            .boxed()
    }
}
