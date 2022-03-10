use super::context::get_pool;
use super::endpoints::{graphql_endpoints, subscription_endpoints};
use super::graphql::schema;
use actix_web::{middleware, web::Data, App, HttpServer};
use env_logger;
use std::{env, io};

#[tokio::main]
pub async fn init() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    // Instantiate a new connection pool
    let pool = get_pool("DATABASE_URL");
    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(schema()))
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
            .configure(subscription_endpoints)
    })
    .bind("127.0.0.1:2222")?
    .run()
    .await
}
