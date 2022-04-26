use crate::models::{ChatMessage, UserInfo};
use diesel::{r2d2::ConnectionManager, pg::PgConnection};
use dotenv::dotenv;
use r2d2::Pool;
use std::env;
use tokio::sync::broadcast;

// The Postgres-specific connection pool managing all database connections.
type PostgresPool = Pool<ConnectionManager<PgConnection>>;

fn get_pool(env: &str) -> PostgresPool {
    // TODO: pass the connection URL into this function rather than extracting
    // it from the environment within this function
    dotenv().ok();
    let url = env::var(env).expect("environment value must be set");
    // A connection pool maintains a set of open connections to a database, handing them out for repeated use.
    let mgr = ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool") // TODO: handle errors
}

// The GraphQL context, which needs to provide everything necessary for
// interacting with the database.
#[derive(Clone)]
pub struct GraphQLContext {
    pub pool: PostgresPool,
    pub sender: broadcast::Sender<UserInfo>,
    pub chat_message_sender: broadcast::Sender<ChatMessage>,
}

impl GraphQLContext {
    pub fn new() -> Self {
        let pool = get_pool("DATABASE_URL");
        let (sender, _) = broadcast::channel::<UserInfo>(100);
        let (chat_message_sender, _) = broadcast::channel::<ChatMessage>(100);
        GraphQLContext {
            pool,
            sender,
            chat_message_sender,
        }
    }
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}
