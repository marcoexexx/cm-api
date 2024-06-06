use serde::Serialize;

#[derive(Serialize)]
pub struct HttpListResponse<T: Serialize> {
  pub result: Vec<T>,
  pub count: usize,
}

#[derive(Serialize)]
pub struct HttpResponse {
  pub message: String,
  pub status_code: u16,
}
