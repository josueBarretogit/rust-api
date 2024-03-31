use axum::{extract::{self, FromRequest, Request}, response::IntoResponse, Json};
use serde::Serialize;

pub mod book_controller;

pub trait Controller<T> 
where
    T : Serialize 
{

    async fn get_models() -> impl IntoResponse;

    async fn create_model(body : axum::extract::Json<T> ) -> impl IntoResponse;

    async fn get_by_id(id : axum::extract::Path<u32> ) -> impl IntoResponse;
    
}


