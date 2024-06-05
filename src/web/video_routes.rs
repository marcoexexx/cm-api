use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

pub fn routes() -> Router {
  Router::new().route("/api/v1/movies", get(get_random_videos))
}

async fn get_random_videos() -> impl IntoResponse {
  Json(json!({
    "videos": []
  }))
}
