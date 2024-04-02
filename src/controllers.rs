use std::error::Error;

use axum::{async_trait, extract::{self, FromRequest, Request, State}, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::Value;



pub mod book_controller;
pub mod customer_controller;

pub trait Controller<T, K>
where
    T : Serialize,
    K : Clone
{

    async fn handle_get_models(state: State<K>) -> Result<Json<Vec<T>>, (StatusCode, Json<Value>)>;


    async fn handle_create_model(body : axum::extract::Json<T> ) -> impl IntoResponse;


}




