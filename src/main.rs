mod error;
mod model;
mod service;

use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};

use error::Error;
use model::{Video, VideoLinkInfo};
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
    let router = Router::new()
        .route("/videos", get(get_videos_handler))
        .route("/", get(get_download_link_handler));

    Ok(router.into())
}
