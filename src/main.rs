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
mod utils;
mod web;

async fn fallback_handler() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  dotenv().ok();

  let origins = [
    "http://127.0.0.1:33037".parse::<HeaderValue>().unwrap(),
    "http://127.0.0.1".parse::<HeaderValue>().unwrap(),
    "http://localhost:33037".parse::<HeaderValue>().unwrap(),
    "http://localhost".parse::<HeaderValue>().unwrap(),
  ];

  let cors = tower_http::cors::CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(origins);

  let api_router = web::movie_routes::routes()
    .route_layer(axum::middleware::from_fn(web::mw_auth::mw_require_auth));

  let router = Router::new()
    .fallback(fallback_handler)
    .nest("/api/v1", api_router)
    .layer(axum::middleware::map_response(
      web::mw_response_mapper::mw_response_mapper,
    ))
    .layer(axum::middleware::from_fn(web::mw_auth::mw_ctx_resolver))
    .layer(cors);

  Ok(router.into())
}
