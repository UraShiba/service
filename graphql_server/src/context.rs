use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;
use tokio::sync::broadcast;

use super::models::UserInfo;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool(env: &str) -> PostgresPool {
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
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}