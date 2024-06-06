use axum::http::HeaderValue;
use axum::Router;
use reqwest::Method;

mod ctx;
mod error;
mod model;
mod response;
mod service;
mod web;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  let origins = [
    "http://127.0.0.1:33037".parse::<HeaderValue>().unwrap(),
    "http://127.0.0.1".parse::<HeaderValue>().unwrap(),
    "http://localhost:33037".parse::<HeaderValue>().unwrap(),
    "http://localhost".parse::<HeaderValue>().unwrap(),
  ];

  let cors = tower_http::cors::CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(origins);

  let router = Router::new()
    .merge(web::movie_routes::routes())
    .layer(axum::middleware::map_response(
      web::mw_response_mapper::mw_response_mapper,
    ))
    .layer(cors);

  Ok(router.into())
}
