use reqwest::Client;
use scraper::selectable::Selectable;

use crate::model::{self, VideoLinkInfo};

pub async fn get_video_link_info(url: &str) -> Result<Vec<VideoLinkInfo>, crate::Error> {
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .expect("Unable to `GET` request.")
        .text()
        .await
        .expect("unable to get text from response.");

    let document = scraper::Html::parse_document(&response);

    let li_selector = scraper::Selector::parse("li.elemento").expect("Unable to parse li selector");
    let a_selector = scraper::Selector::parse("a").expect("Unable to parse a selector");
    let protocol_selector =
        scraper::Selector::parse("span.b").expect("Unable to parse protocol selector");
    let size_selector = scraper::Selector::parse("span.c").expect("Unable to parse size selector");
    let quality_selector =
        scraper::Selector::parse("span.d").expect("Unable to parse quality selector");

    let mut movies = Vec::new();

    for li in document.select(&li_selector) {
        if let Some(a) = li.select(&a_selector).next() {
            let link = a
                .value()
                .attr("href")
                .map_or(String::from("NotFound"), String::from);
            let protocol = a
                .select(&protocol_selector)
                .next()
                .map_or(String::from("NotFound"), |el| {
                    el.inner_html().trim().to_string()
                })
                .to_string();
            let size = a
                .select(&size_selector)
                .next()
                .map_or(String::from("NotFound"), |el| {
                    el.inner_html().trim().to_string()
                })
                .to_string();
            let quality = a
                .select(&quality_selector)
                .next()
                .map_or(String::from("NotFound"), |el| {
                    el.inner_html().trim().to_string()
                })
                .to_string();

            movies.push(model::VideoLinkInfo::new(&protocol, &size, &quality, &link));
        }
    }

    Ok(movies)
}
