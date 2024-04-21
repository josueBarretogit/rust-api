use std::{default, error::Error, future::Future, pin::Pin, sync::Arc};

use axum::{
    extract::{self, FromRequest, Json, Multipart, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

use serde::Serialize;
use serde_json::{json, Value};

use crate::helpers::types::ApiResponse;

pub mod book_controller;
pub mod customer_controller;
pub mod file_controller;

pub type StateController<T> = State<Arc<T>>;

/// T: model, K: appstate
pub trait Controller<T, K, CREATEDTO>
where
    T: Serialize,
    K: Clone,
{
    async fn handle_get_models(state: StateController<K>) -> ApiResponse;

    async fn handle_create_model(state: StateController<K>, body: Json<CREATEDTO>) -> ApiResponse;
}

pub trait FileHandler {
    async fn handle_upload(mult: Multipart) -> impl IntoResponse;

    async fn handle_delete_file(mult: Multipart) -> impl IntoResponse;
}
