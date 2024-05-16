mod error;
mod model;
mod service;

use axum::{extract::Query, http::HeaderValue, routing::get, Json, Router};

use error::Error;
use model::{Video, VideoLinkInfo};
use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize)]
struct DownloadLinkQuery {
    slug: String,
}

async fn get_download_link_handler(
    Query(DownloadLinkQuery { slug }): Query<DownloadLinkQuery>,
) -> Json<Vec<VideoLinkInfo>> {
    let movie = service::get_info::get_video_link_info(&slug)
        .await
        .expect("Unable to get video info");

    Json(movie)
}

async fn get_videos_handler() -> Json<Vec<Video>> {
    let random_movies = service::get_movies::get_random_movies()
        .await
        .expect("Unable to get random movies");

    Json(random_movies)
}

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
        // .allow_origin(tower_http::cors::Any);
        .allow_origin(origins);

    let router = Router::new()
        .route("/", get(get_videos_handler))
        .route("/download", get(get_download_link_handler))
        .layer(cors);

    Ok(router.into())
}
