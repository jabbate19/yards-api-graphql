use std::sync::Arc;

use actix_cors::Cors;
use actix_web::http::Error;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use diesel::pg::Pg;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use juniper::GraphQLObject;

mod schemas;
use crate::schemas::context::{get_pool, GraphQLContext, PostgresPool};
use crate::schemas::schema::{create_schema, Schema};
mod db;
/// Playground
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

use crate::db::db::address::dsl::*;
use crate::db::db::device::{self, dsl::*};
use crate::db::db::interface::dsl::*;
use crate::db::db::iprange::dsl::*;
use crate::db::db::staticaddress::dsl::*;
use crate::db::models::{Address, Device, IPRange, Interface, StaticAddress};
use diesel::{debug_query, prelude::*};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,)]
// struct Ligma3 {
//     #[serde(flatten)]
//     address: Address,
//     #[serde(flatten)]
//     iprange: IPRange,
//     #[serde(flatten)]
//     staticaddress: Option<StaticAddress>
// }

// #[derive(Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,)]
// struct Ligma2 {
//     #[serde(flatten)]
//     interface: Interface,
//     #[serde(flatten)]
//     addresses: Vec<Ligma3>,
// }

// #[derive(Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,)]
// struct Ligma {
//     #[serde(flatten)]
//     device: Device,
//     #[serde(flatten)]
//     interfaces: Vec<Ligma2>,
// }

/// Endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    // The DB connection pool
    app_data: web::Data<PostgresPool>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let pool = app_data.into_inner();
    // Instantiate a context
    let ctx = GraphQLContext {
        pool: (*pool).clone(),
    };

    let schema = create_schema();
    log::info!("Ligma");
    // Handle the incoming request and return a string result (or error)
    let res = data.execute(&schema, &ctx).await;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(res))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let pool = get_pool();

    let schema = std::sync::Arc::new(create_schema());

    let port = 9000;

    log::info!("Starting on Port: http://localhost:{}", port);
    log::info!("Playground running on: http://localhost:{}/graphiql", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .app_data(web::Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
}
