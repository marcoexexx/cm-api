use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use crate::error::Result;
use crate::response::HttpListResponse;
use crate::service;

pub fn routes() -> Router {
  Router::new()
    .route("/api/v1/movies", get(get_random_movies))
    .route("/api/v1/series", get(get_random_series))
    .route("/api/v1/download/:slug", get(get_download_info))
}

async fn get_random_movies() -> Result<Json<impl Serialize>> {
  let movies = service::get_movies::get_random_movies().await?;

  let response = HttpListResponse {
    result: movies.clone(),
    count: movies.len(),
  };

  Ok(Json(response))
}

async fn get_random_series() -> Result<Json<impl Serialize>> {
  if true {
    return Err(crate::error::Error::BackendServiceFail);
  }

  Ok(Json(serde_json::json!({})))
}

async fn get_download_info(Path(slug): Path<String>) -> Result<Json<impl Serialize>> {
  let download_links = service::get_info::get_download_info(&slug).await?;

  let response = HttpListResponse {
    result: download_links.clone(),
    count: download_links.len(),
  };

  Ok(Json(response))
}
