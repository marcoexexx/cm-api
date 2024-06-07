#![allow(unused)]

use axum::http::HeaderValue;
use axum::response::IntoResponse;
use axum::Router;
use dotenv::dotenv;
use reqwest::{Method, StatusCode};

mod ctx;
mod error;
mod model;
mod response;
mod service;
mod tests;
mod utils;
mod web;

async fn ping_pong_handler() -> impl IntoResponse {
  (StatusCode::OK, "PONG")
}

async fn fallback_handler() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  dotenv().ok();

  let cors = tower_http::cors::CorsLayer::permissive();

  let api_router = web::movie_routes::routes()
    .route_layer(axum::middleware::from_fn(web::mw_auth::mw_require_auth));

  let router = Router::new()
    .route("/ping", axum::routing::get(ping_pong_handler))
    .fallback(fallback_handler)
    .nest("/api/v1", api_router)
    .layer(axum::middleware::map_response(
      web::mw_response_mapper::mw_response_mapper,
    ))
    .layer(axum::middleware::from_fn(web::mw_auth::mw_ctx_resolver))
    .layer(cors);

  Ok(router.into())
}
