use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoLinkInfo {
    pub protocol: String,
    pub link: String,
    pub size: String,
    pub quality: String,
}
impl VideoLinkInfo {
    pub fn new(protocol: &str, link: &str, size: &str, quality: &str) -> VideoLinkInfo {
        VideoLinkInfo {
            protocol: String::from(protocol),
            link: String::from(link),
            size: String::from(size),
            quality: String::from(quality),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub title: String,
    pub image: String,
    pub links: Vec<VideoLinkInfo>
}
