use scraper::selectable::Selectable;
use scraper::Selector;

use super::get_selectors;
use crate::error::Error;
use crate::model::video::Video;

pub async fn get_random_movies() -> Result<Vec<Video>, Error> {
  let mut new_release_movies = Vec::new();

  let selectors = get_selectors();

  let new_release_selector = Selector::parse(&selectors.new_release).expect("Unable to parse");

  let document = super::get_document("").await;

  for movie_el in document.select(&new_release_selector) {
    let cm_link = movie_el
      .select(&Selector::parse("a").expect("Unable to parse `a`"))
      .next()
      .unwrap()
      .value()
      .attr("href")
      .map_or(String::new(), String::from);

    let title = movie_el
      .select(&Selector::parse("span.ttps").expect("Unable to parse `span.ttps`"))
      .next()
      .unwrap()
      .text()
      .next()
      .unwrap()
      .to_owned();

    let poster = movie_el
      .select(&Selector::parse("a img").expect("Unable to parse `span.ttps`"))
      .next()
      .unwrap()
      .value()
      .attr("src")
      .map_or(String::new(), String::from);

    new_release_movies.push(Video {
      cm_link: cm_link.clone(),
      slug: cm_link.split("/").nth(3).unwrap_or("").to_owned(),
      title,
      poster,
      photos: Vec::new(),
      description: String::default(),
    })
  }

  Ok(new_release_movies)
}
