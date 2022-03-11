use super::context::{GraphQLContext, PostgresPool};
use super::graphql::Schema;
use super::models::UserInfo;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::http::playground::playground_source;
use juniper_actix::graphql_handler;
use juniper_actix::subscriptions::subscriptions_handler;
use juniper_graphql_ws::ConnectionConfig;
use std::time::Duration;
use tokio::sync::broadcast;

// The configuration callback that enables us to add the /graphql route
// to the actix-web server.
pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    config
        .route("/graphql", web::post().to(graphql))
        .route("/graphql", web::get().to(graphql_playground));
}

pub fn subscription_endpoints(config: &mut web::ServiceConfig) {
    config.route("/subscriptions", web::get().to(subscriptions));
}

async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    // The DB connection pool
    pool: web::Data<PostgresPool>,
    // The GraphQL schema
    schema: web::Data<Schema>,
    sender: web::Data<broadcast::Sender<UserInfo>>,
) -> Result<HttpResponse, Error> {
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
        sender: sender.get_ref().to_owned(),
    };
    let schema = schema.into_inner();
    let config = ConnectionConfig::new(ctx);
    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions_handler(req, stream, schema, config).await
}

// The GraphQL Playground route.
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql", Some("/subscriptions")))
}

// The core handler that provides all GraphQL functionality.
async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    // The DB connection pool
    pool: web::Data<PostgresPool>,
    // The GraphQL schema
    schema: web::Data<Schema>,
    sender: web::Data<broadcast::Sender<UserInfo>>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
        sender: sender.get_ref().to_owned(),
    };
    graphql_handler(&schema, &ctx, req, payload).await
}
