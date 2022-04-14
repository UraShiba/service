use super::context::GraphQLContext;
use super::schema::chat_message;
use super::schema::user_info;
use juniper::graphql_object;

#[derive(Queryable, Debug, Insertable, Clone)]
#[table_name = "user_info"]
pub struct UserInfo {
    pub user_id: i32,
    pub user_name: String,
    pub login_name: String,
    pub pass: String,
}

#[graphql_object(context = GraphQLContext)]
impl UserInfo {
    fn user_id(&self) -> &i32 {
        &self.user_id
    }
    fn user_name(&self) -> &str {
        &self.user_name
    }
    fn login_name(&self) -> &str {
        &self.login_name
    }
    fn pass(&self) -> &str {
        &self.pass
    }
}

#[derive(Queryable, Debug, Insertable, Clone)]
#[table_name = "chat_message"]
pub struct ChatMessage {
    pub message_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub message_text: String,
    pub sent_datetime: String,
}

#[graphql_object(context = GraphQLContext)]
impl ChatMessage {
    fn message_id(&self) -> &i32 {
        &self.message_id
    }
    fn from_user_id(&self) -> &i32 {
        &self.from_user_id
    }
    fn to_user_id(&self) -> &i32 {
        &self.to_user_id
    }
    fn message_text(&self) -> &str {
        &self.message_text
    }
    fn sent_datetime(&self) -> &str {
        &self.sent_datetime
    }
}
