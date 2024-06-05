use axum::response::IntoResponse;
use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
  BackendServiceFail,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    response.extensions_mut().insert(self);

    response
  }
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, &str) {
    match self {
      Self::BackendServiceFail => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal backend service error",
      ),
    }
  }
}
