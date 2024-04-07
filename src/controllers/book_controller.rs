use axum::middleware::Next;
use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};
use axum::{extract::*, BoxError};
use core::panic;
use serde_json::{json, Value};
use sqlx::types::chrono;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::ExtractJwt;
use crate::{models::book_models::Books, repositories::Repository, AppStateBooks, BooksRepository};

pub struct BookController {}

impl super::Controller<Books, AppStateBooks<BooksRepository>> for BookController {
    async fn handle_get_models(
        state: State<Arc<AppStateBooks<BooksRepository>>>,
    ) -> Result<Json<Vec<Books>>, (StatusCode, Json<Value>)> {
        let repository = &state.repository;

        let response = repository.find_all().await;

        match response {
            Ok(books) => Ok(Json(books)),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"response" : "bad request", "details" : e.to_string()})),
            )),
        }
    }

    async fn handle_create_model(Json(body): Json<Books>) -> impl IntoResponse {
        let book = Books::new(body.description, body.title);

        Json(json!({"book:" : book}))
    }
}
