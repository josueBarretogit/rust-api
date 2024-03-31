use axum::{ http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::models::book_models::Books;

#[derive(Clone)]
pub struct BookController {}


impl super::Controller<Books> for BookController {


    async fn get_models() -> Json<Books> {
        let book = Books::new("a new book aaaaa".to_string());

        Json(book)
    }


    async fn create_model(Json(body): Json<Books>) -> impl IntoResponse {

        let book = Books::new(body.description);

        Json(json!({"book:" : book}))
    }

    async fn get_by_id(id : axum::extract::Path<u32> ) -> impl IntoResponse {
        let id_received = id.to_string();

        Json(json!({
        "id" : id_received
        }))
    }
}


    
