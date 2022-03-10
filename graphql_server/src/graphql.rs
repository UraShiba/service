use super::context::GraphQLContext;
use super::models::UserInfo;
use super::schema::user_info::dsl::user_info;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::RootNode;
use juniper::{graphql_object, graphql_subscription, FieldError};
use std::pin::Pin;
use tokio_stream::Stream;

type AccountStream = Pin<Box<dyn Stream<Item = Result<AccountInfo, FieldError>> + Send>>;
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

#[derive(Clone, Debug)]
struct AccountInfo {
    name: String,
    id: String,
    email: String,
    pass: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Account {
    id: String,
    pass: String,
}

#[graphql_object(context = GraphQLContext)]
impl AccountInfo {
    fn name(&self) -> &str {
        &self.name
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn email(&self) -> &str {
        &self.email
    }
    fn pass(&self) -> &str {
        &self.pass
    }
}

pub struct Query;
#[graphql_object(context = GraphQLContext)]
impl Query {
    async fn getAccountByQuery(
        context: &GraphQLContext,
        id: String,
        pass: String,
    ) -> Option<AccountInfo> {
        let conn: &PgConnection = &context.pool.get().unwrap();
        let results = user_info
            .find(id)
            .load::<UserInfo>(&conn)
            .expect("Error loading customer");
        let query = Account {
            id: id.to_string(),
            pass: pass.to_string(),
        };
        context.get_account_by_name(&query).await
    }
}
pub struct Mutation;
#[graphql_object(context = GraphQLContext)]
impl Mutation {
    async fn signUp(context: &GraphQLContext, user: UserInfo) -> AccountInfo {
        let conn: &PgConnection = &context.pool.get().unwrap();
        diesel::insert_into(user_info)
            .values(&user)
            .execute(&conn)
            .expect("Error saving new Users");
        context.register_user(&name, &id, &email, &pass).await;
        AccountInfo {
            name,
            id,
            email,
            pass,
        }
    }
}

pub struct Subscription;
#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn subscribe_account(context: &GraphQLContext) -> AccountStream {
        let conn: &PgConnection = &context.pool.get().unwrap();
        tokio_stream::wrappers::BroadcastStream::new(context.sender.subscribe())
            .map(|result| match result {
                Ok(account) => Ok(account),
                Err(err) => Err(FieldError::from(err.to_string())),
            })
            .boxed()
    }
}
