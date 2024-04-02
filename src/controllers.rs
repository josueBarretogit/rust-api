use axum::{async_trait, extract::{self, FromRequest, Request, State}, response::IntoResponse, Json};
use serde::Serialize;

use crate::routes::book_routes::AppStateBooks;


pub mod book_controller;

pub trait Controller<T>
where
    T : Serialize
{

    async fn handle_get_models(state: State<AppStateBooks>) -> impl IntoResponse;


    async fn handle_create_model(body : axum::extract::Json<T> ) -> impl IntoResponse;


}




