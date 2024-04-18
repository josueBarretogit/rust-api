use std::{default, error::Error, future::Future, pin::Pin, sync::Arc};

use axum::{
    extract::{self, FromRequest, Multipart, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json
};

use serde::Serialize;
use serde_json::{json, Value};


pub mod book_controller;
pub mod customer_controller;
pub mod file_controller;



pub type StateController<T> = State<Arc<T>>;

/// T: model, K: appstate
pub trait Controller<T, K>
where
    T: Serialize,
    K: Clone,
{
    async fn handle_get_models( state: StateController<K>) -> Result<Json<Vec<T>>, impl IntoResponse>;

    async fn handle_create_model(body: axum::extract::Json<T>) -> impl IntoResponse;
}

pub trait FileHandler {
    async fn handle_upload(mult: Multipart) -> impl IntoResponse;

    async fn handle_delete_file(mult: Multipart) -> impl IntoResponse;
}

