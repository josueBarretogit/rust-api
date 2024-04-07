use std::{error::Error, future::Future, pin::Pin, sync::Arc};

use axum::{
    async_trait,
    body::Body,
    extract::{self, FromRequest, Multipart, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use future_utils::BoxFuture;
use serde::Serialize;
use serde_json::Value;

use crate::ExtractJwt;

pub mod book_controller;
pub mod customer_controller;
pub mod file_controller;

/// T: model, K: appstate
pub trait Controller<T, K>
where
    T: Serialize,
    K: Clone,
{
    async fn handle_get_models(
        state: State<Arc<K>>,
    ) -> Result<Json<Vec<T>>, (StatusCode, Json<Value>)>;

    async fn handle_create_model(body: axum::extract::Json<T>) -> impl IntoResponse;
}

pub trait FileHandler {
    async fn handle_upload(mult: Multipart) -> impl IntoResponse;

    async fn handle_delete_file(mult: Multipart) -> impl IntoResponse;
}

pub async fn tes(ExtractJwt(jwt): ExtractJwt) -> impl IntoResponse {
    println!("the jwt is: {}", jwt.to_str().unwrap());

    "hola middle"
}
