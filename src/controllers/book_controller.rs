use std::sync::Arc;

use axum::middleware::Next;
use axum::{ debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use axum::extract::*;
use serde_json::{json, Value};

use crate::ExtractJwt;
use crate::{models::book_models::Books, repositories::Repository, AppStateBooks,  BooksRepository};



pub struct BookController {}



impl super::Controller<Books, AppStateBooks<BooksRepository>> for BookController {

    async fn handle_get_models(state: State<Arc<AppStateBooks<BooksRepository>>>, req : axum::extract::Request) -> Result<Json<Vec<Books>>, (StatusCode, Json<Value>)> {

        let repository = &state.repository;



        let response = repository.find_all().await;

        match response {
            Ok(books) => Ok(Json(books)),
            Err(e) => Err((StatusCode::BAD_REQUEST, Json(json!({"response" : "bad request", "details" : e.to_string()}))))
        }

    }

    async fn handle_create_model( Json(body): Json<Books>) -> impl IntoResponse {

        let book = Books::new(body.description, body.title);

        Json(json!({"book:" : book}))
    }


}

impl BookController {
    
    pub async fn tes(ExtractJwt(jwt) : ExtractJwt) -> impl IntoResponse {

        println!("the jwt is: {}", jwt.to_str().unwrap());

        "hola middle"
    }
}


