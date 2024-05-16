pub mod get_info;
pub mod get_movies;

use reqwest::Client;
use scraper::Html;

pub const BASE_URL: &str = "https://www.channelmyanmar.to";

async fn get_document(url: &str) -> Html {
    let client = Client::new();

    let response = client
        .get(&format!("{BASE_URL}/{url}"))
        .send()
        .await
        .expect("Failed to request url.")
        .text()
        .await
        .expect("Failed to parse reposne text.");

    scraper::Html::parse_document(&response)
}

pub struct Selectors {
    pub new_release: String,
}

pub fn get_selectors() -> Selectors {
    Selectors {
        new_release: String::from("#slider2.owl-carousel.owl-theme .item"),
    }
}
