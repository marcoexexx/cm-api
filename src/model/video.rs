use serde::Serialize;

pub struct DownloadInfo {
  pub protocol: String,
  pub link: String,
  pub size: String,
  pub quality: String,
}

#[derive(Serialize)]
pub struct Video {
  pub title: String,
  pub slug: String,
  pub cm_link: String,
  pub description: String,
  pub poster: String,
  pub photos: Vec<String>,
}
