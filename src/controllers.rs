use std::{error::Error, future::Future, pin::Pin, sync::Arc};

use axum::{
    extract::{self, FromRequest, Multipart, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Serialize;
use serde_json::Value;

pub mod book_controller;
pub mod customer_controller;
pub mod file_controller;



pub struct OkRes<T> 
    where T: Serialize
{
    success: bool,
    status_code: StatusCode,
    data : Json<T>
} 

pub enum ApiResponse<T> 
    where T: Serialize

{
    Ok(OkRes<T>),
    BadRequest,
    InternalServerError(anyhow::Error)
}



pub type StateController<T> = State<Arc<T>>;
pub type GetAllResponse<T> = Result<Json<Vec<T>>, (StatusCode, Json<Value>)>;

/// T: model, K: appstate
pub trait Controller<T, K>
where
    T: Serialize,
    K: Clone,
{
    async fn handle_get_models( state: StateController<K>) -> GetAllResponse<T>;

    async fn handle_create_model(body: axum::extract::Json<T>) -> impl IntoResponse;
}

pub trait FileHandler {
    async fn handle_upload(mult: Multipart) -> impl IntoResponse;

    async fn handle_delete_file(mult: Multipart) -> impl IntoResponse;
}

