use axum::{extract::{self, FromRequest, Request}, response::IntoResponse, Json};
use serde::Serialize;

pub mod book_controller;

pub trait Controller<T>
where
    T : Serialize
{

    async fn handle_get_models() -> impl IntoResponse;


    async fn handle_create_model(body : axum::extract::Json<T> ) -> impl IntoResponse;

}


