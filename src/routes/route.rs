use axum::Json;


use crate::models::book_models::{self, Books};

pub async fn get_books() -> Json<Books> {

    let book = book_models::Books::new("a description".to_string());

    Json(book)
}
