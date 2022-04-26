use actix_web::{middleware, web::Data, App, HttpServer};
use env_logger;
use graphql_server::context::GraphQLContext;
use graphql_server::endpoints::{graphql_endpoints, subscription_endpoints};
use graphql_server::graphql::schema;
use std::{env, io};

#[tokio::main]
pub async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    // Instantiate a new connection pool
    let context = GraphQLContext::new();
    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(context.clone()))
            .app_data(Data::new(schema()))
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
            .configure(subscription_endpoints)
    })
    .bind("0.0.0.0:5050")?
    .run()
    .await
}
