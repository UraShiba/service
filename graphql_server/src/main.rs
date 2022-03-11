use actix_web::{middleware, web::Data, App, HttpServer};
use env_logger;
use graphql_server::context::get_pool;
use graphql_server::endpoints::{graphql_endpoints, subscription_endpoints};
use graphql_server::graphql::schema;
use graphql_server::models::UserInfo;
use std::{env, io};
use tokio::sync::broadcast;

#[tokio::main]
pub async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    // Instantiate a new connection pool
    let pool = get_pool("DATABASE_URL");
    let (sender, _) = broadcast::channel::<UserInfo>(100);
    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema()))
            .app_data(Data::new(sender.clone()))
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
            .configure(subscription_endpoints)
    })
    .bind("127.0.0.1:2222")?
    .run()
    .await
}
