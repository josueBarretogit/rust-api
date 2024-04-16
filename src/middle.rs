use axum::{
    async_trait, extract::{FromRequest, FromRequestParts, Multipart, Request}, http::{header, request::Parts, HeaderMap, HeaderName, HeaderValue, StatusCode}, middleware::Next, response::IntoResponse, BoxError, Json, RequestExt
};

use regex::*;
use serde_json::{json, Value};

pub async fn authorize(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
    let auth = req.headers().get(header::AUTHORIZATION);
    match auth {
        Some(a) => {
            let res = next.run(req).await;
            println!("aslkdjsal");
            Ok(res)
        }
        None => Err((StatusCode::UNAUTHORIZED, "UNAUTHORIZED")),
    }
}


pub struct ExtractJwt(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractJwt
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {


        if let Some(jwt) = parts.headers.get("jwt") {
            return Ok(ExtractJwt(jwt.clone()));
        }
        Err((
            StatusCode::UNAUTHORIZED,
            "`json web token not found` header is missing",
        ))
    }
}

pub fn ok_response(statuscode:  StatusCode, message :  Json<Value>) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"response" : "ok"})))
}
