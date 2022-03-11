use super::context::GraphQLContext;
use super::schema::user_info;
use juniper::graphql_object;

#[derive(Queryable, Debug, Insertable,Clone)]
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
