use crate::service;
use crate::utils;

#[tokio::test]
async fn get_random_movies() {
  let html = service::get_document("").await;
  utils::write_html(&html);
}
