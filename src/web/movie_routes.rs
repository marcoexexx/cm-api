use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};

use crate::error::{Error, Result};
use crate::service;

pub fn routes() -> Router {
  Router::new().route("/api/v1/movies", get(get_random_movies))
}

async fn get_random_movies() -> Result<Json<Value>> {
  let movies = service::get_movies::get_random_movies()
    .await
    .map_err(|_| Error::BackendServiceFail)?;

  Ok(Json(json!({
    "movies": movies
  })))
}
