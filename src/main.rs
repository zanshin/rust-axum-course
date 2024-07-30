#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;

use crate::model::ModelController;

use axum::extract::{Path, Query};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use axum::response::{Html, IntoResponse, Response};

use serde::Deserialize;

use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController.
    let mc = ModelController::new().await?;

    // Having the route_layer here scopes it to the web routes
    // If it were in the list of routes below it would effect all the other routes
    let routes_apis = web::routes_tickets::routes(mc.clone())
        // .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all= Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:    --- Start Server {{{
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server }}}

    Ok(())

}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Routes Hello {{{
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Mark`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello/Mark`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"))
}

// endregion: --- Handler Hello }}}
