use axum::extract::{Path, Query};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::response::HttpListResponse;
use crate::service;

#[derive(Deserialize)]
struct SearchMovieQuery {
  q: String,
  page: Option<usize>,
}

pub fn routes() -> Router {
  Router::new()
    .route("/movies", get(get_random_movies))
    .route("/series", get(get_random_series))
    .route("/search", get(search_movies))
    .route("/download/:slug", get(get_download_info))
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
  Ok(Json(serde_json::json!({})))
}

async fn search_movies(Query(query): Query<SearchMovieQuery>) -> Result<Json<impl Serialize>> {
  let movies = service::search_movies::search_movies(&query.q, query.page).await?;

  let response = HttpListResponse {
    result: movies.clone(),
    count: movies.len(),
  };

  Ok(Json(response))
}

async fn get_download_info(Path(slug): Path<String>) -> Result<Json<impl Serialize>> {
  let download_links = service::get_info::get_download_info(&slug).await?;

  let response = HttpListResponse {
    result: download_links.clone(),
    count: download_links.len(),
  };

  Ok(Json(response))
}
