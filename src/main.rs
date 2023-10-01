#![allow(unused)]

pub use self::error::{Error, Result};
use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, Route},
    Router,
};

use serde::Deserialize;
use tower_cookies::{CookieManager, CookieManagerLayer};
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = model::ModelController::new().await?;
    let routers_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_wrapper))
        .layer(middleware::map_response(main_response_wrapper2))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    println!("->> listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routers_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_wrapper(res: Response) -> Response {
    println!("main_response_wrapper");
    res
}

async fn main_response_wrapper2(res: Response) -> Response {
    println!("main_response_wrapper2");
    res
}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("handler hello - {params:?}");
    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("{}", name);
    Html(format!("Hello <strong>{name}</strong>"))
}
