use scraper::selectable::Selectable;

use crate::error::Result;
use crate::model::video::Video;
use crate::service;

pub async fn search_movies(query: &str, page: Option<usize>) -> Result<Vec<Video>> {
  let mut movies = Vec::new();

  let document = service::get_document(&format!(
    "https://www.channelmyanmar.to/page/{page}/?s={query}",
    page = page.unwrap_or(1)
  ))
  .await;

  let item_selector =
    scraper::selector::Selector::parse(".item").expect("Unable to parse item selector");
  let a_selector = scraper::selector::Selector::parse("a").expect("Unable to parse a selector");
  let img_selector =
    scraper::selector::Selector::parse("img").expect("Unable to parse img selector");
  let fixyear_selector =
    scraper::selector::Selector::parse("div .fixyear").expect("Unable to parse fixyear selector");
  let h2_selector = scraper::selector::Selector::parse("h2").expect("Unable to parse h2 selector");

  for item_el in document.select(&item_selector) {
    let a_el = item_el
      .select(&a_selector)
      .next()
      .expect("Unable to parse link");

    let cm_link = a_el
      .attr("href")
      .map(String::from)
      .unwrap_or(String::default());

    let poster = a_el
      .select(&img_selector)
      .next()
      .expect("Unable to parse poster")
      .attr("src")
      .map(String::from)
      .unwrap_or(String::default());

    // let title = item_el
    //   .select(&h2_selector)
    //   .next()
    //   .expect("Unable to parse fixyear")
    //   .text()
    //   .next()
    //   .map(String::from)
    //   .unwrap_or(String::default());

    movies.push(Video {
      title: String::default(),
      slug: String::default(),
      cm_link,
      poster,
    });
  }

  Ok(movies)
}
