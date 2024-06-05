use scraper::selectable::Selectable;

use crate::error::Error;
use crate::model::video::DownloadInfo;

pub async fn get_download_info(url: &str) -> Result<Vec<DownloadInfo>, Error> {
  let document = super::get_document(url).await;

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
        .unwrap()
        .text()
        .nth(1)
        .unwrap()
        .split("\n")
        .nth(1)
        .unwrap()
        .to_string();

      let size = a
        .select(&size_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string();

      let quality = a
        .select(&quality_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .to_string();

      movies.push(DownloadInfo {
        protocol,
        link,
        size,
        quality,
      });
    }
  }

  Ok(movies)
}
