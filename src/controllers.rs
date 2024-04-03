use std::{error::Error, future::Future, pin::Pin};

use axum::{async_trait, body::Body, extract::{self, FromRequest, Request, State}, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use future_utils::BoxFuture;
use serde::Serialize;
use serde_json::Value;



pub mod book_controller;
pub mod customer_controller;

/// T: model, K: appstate
pub trait Controller<T, K>
where
    T : Serialize,
    K : Clone
{

    async fn handle_get_models(state: Extension<K>) -> Result<Json<Vec<T>>, (StatusCode, Json<Value>)>;


    async fn handle_create_model(body : axum::extract::Json<T> ) -> impl IntoResponse;



}




