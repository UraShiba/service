pub mod schema;

use schema::chat_message;
use schema::user_info;
use super::context::GraphQLContext;
use juniper::graphql_object;

#[derive(Debug, Clone)]
pub struct Response {
    pub token: String
}

#[graphql_object(context = GraphQLContext)]
impl Response {
    fn token(&self) -> &String {
        &self.token
    }
}

#[derive(Queryable, Debug, Insertable, Clone)]
#[table_name = "user_info"]
pub struct UserInfo {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub pass: String,
}

#[graphql_object(context = GraphQLContext)]
impl UserInfo {
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn user_name(&self) -> &str {
        &self.user_name
    }
    fn email(&self) -> &str {
        &self.email
    }
    fn pass(&self) -> &str {
        &self.pass
    }
}

#[derive(Queryable, Debug, Insertable, Clone)]
#[table_name = "chat_message"]
pub struct ChatMessage {
    pub message_id: i32,
    pub from_user_id: String,
    pub to_user_id: String,
    pub message_text: String,
    pub sent_date_time: String,
}

#[graphql_object(context = GraphQLContext)]
impl ChatMessage {
    fn message_id(&self) -> &i32 {
        &self.message_id
    }
    fn from_user_id(&self) -> &str {
        &self.from_user_id
    }
    fn to_user_id(&self) -> &str {
        &self.to_user_id
    }
    fn message_text(&self) -> &str {
        &self.message_text
    }
    fn sent_date_time(&self) -> &str {
        &self.sent_date_time
    }
}
