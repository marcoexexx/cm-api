use scraper::{selectable::Selectable, Selector};

use super::get_selectors;
use crate::{error, model};

pub async fn get_random_movies() -> Result<Vec<model::Video>, error::Error> {
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

        let image = movie_el
            .select(&Selector::parse("a img").expect("Unable to parse `span.ttps`"))
            .next()
            .unwrap()
            .value()
            .attr("src")
            .map_or(String::new(), String::from);

        new_release_movies.push(model::Video {
            cm_link,
            title,
            image,
        })
    }

    Ok(new_release_movies)
}
