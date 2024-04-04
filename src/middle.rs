use axum::{extract::{FromRequest, Request}, http::{ HeaderName, HeaderValue,  StatusCode}, middleware::Next, response::IntoResponse, BoxError, RequestExt};

use crate::models::book_models::Books;


pub async fn set_a_header(req :   Request, next : Next) -> impl IntoResponse {

    let mut res = next.run(req).await;

    res.headers().insert("another", HeaderValue::from_static("alskdjsalk"));

    println!("run a middle");

    res

}
