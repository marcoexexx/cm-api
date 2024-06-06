use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::{error::Error, response::HttpResponse};

pub async fn mw_response_mapper(res: Response) -> Response {
  let service_error = res.extensions().get::<Error>();
  let client_error = service_error.map(|se| se.client_status_and_error());

  let error_response = client_error.map(|(status_code, msg)| {
    let body = HttpResponse {
      status_code: status_code.as_u16(),
      message: String::from(msg),
    };

    (status_code, Json(body)).into_response()
  });

  error_response.unwrap_or(res)
}
