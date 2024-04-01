
use axum::{  response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::models::book_models::Books;


#[derive(Clone)]
pub struct BookController {}


impl super::Controller<Books> for BookController {



    async fn handle_get_models() -> Json<Books> {
        let book = Books::new("a new book aaaaa".to_string());

        Json(book)
    }

    
    async fn handle_create_model( Json(body): Json<Books>) -> impl IntoResponse {

        let book = Books::new(body.description);

        Json(json!({"book:" : book}))
    }



}


impl BookController {

    pub fn new() -> Self {
        BookController {  }
    }
}

    
