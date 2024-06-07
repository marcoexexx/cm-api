use axum::async_trait;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, Request};
use axum::middleware::Next;
use axum::response::Response;

use crate::ctx::Ctx;
use crate::error::{Error, Result};
use crate::service::auth;
use crate::utils::decode_base64;

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
  // ctx?;

  Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
  headers: HeaderMap,
  mut req: Request<Body>,
  next: Next,
) -> Result<Response> {
  let auth_header = headers.get("x-auth-token").ok_or(Error::AuthFailNoToken);

  let user = auth_header
    .and_then(|token_header| {
      token_header
        .to_str()
        .map_err(|_| Error::AuthFailInvalidToken)
    })
    .and_then(|token| decode_base64(token).ok_or(Error::AuthFailInvalidToken))
    .and_then(|bytes| String::from_utf8(bytes).map_err(|_| Error::AuthFailInvalidToken))
    .and_then(|token| {
      let mut credential = token.split(":");
      let credential = (
        credential.next().map(String::from),
        credential.next().map(String::from),
      );

      Ok(credential)
    })
    .and_then(|(username, password)| {
      if username.is_none() || password.is_none() {
        return Err(Error::AuthFailInvalidToken);
      }

      Ok((username.unwrap(), password.unwrap()))
    })
    .and_then(|(username, password)| {
      let (db_username, db_password) = auth::get_super_user_from_env();

      if username.eq(&db_username) && password.eq(&db_password) {
        return Ok(Ctx::new(1));
      }

      Err(Error::AuthFailIncorrectUsernameOrPassword)
    });

  req.extensions_mut().insert(user);

  Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
  type Rejection = Error;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    let token = parts
      .extensions
      .get::<Result<Ctx>>()
      .ok_or(Error::AuthFailNoToken)?;

    token.clone()
  }
}
