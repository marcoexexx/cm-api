use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DownloadInfo {
  pub protocol: String,
  pub link: String,
  pub size: String,
  pub quality: String,
}

#[derive(Serialize, Clone)]
pub struct Video {
  pub title: String,
  pub slug: String,
  pub cm_link: String,
  pub poster: String,
}
