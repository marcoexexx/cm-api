mod error;
mod model;
mod service;

use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};

use error::Error;
use model::VideoLinkInfo;
use serde::Deserialize;

// const TEST_VIDEO_URL: &str = "https://channelmyanmar.to/austin-powers-in-goldmember-2002";

#[derive(Deserialize)]
struct DownloadLinkQuery {
    video_link: String,
}

async fn get_download_link_handler(
    Query(DownloadLinkQuery { video_link }): Query<DownloadLinkQuery>,
) -> Json<Vec<VideoLinkInfo>> {
    let movie = service::get_info::get_video_link_info(&video_link)
        .await
        .expect("Unable to get video info");

    Json(movie)
}

async fn get_videos_handler() -> Json<Vec<String>> {
    Json(vec![String::from("Video_1")])
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/videos", get(get_videos_handler))
        .route("/", get(get_download_link_handler));

    Ok(router.into())
}
