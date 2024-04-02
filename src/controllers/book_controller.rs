use axum::{ extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::{models::book_models::Books, repositories::Repository, AppStateBooks};



pub struct BookController {}



impl super::Controller<Books, AppStateBooks> for BookController {

    async fn handle_get_models(state: State<AppStateBooks>) -> Result<Json<Vec<Books>>, (StatusCode, Json<Value>)> {

        let repository = &state.repository;

        let response = repository.find_all().await;
        match response {
            Ok(books) => Ok(Json(books)),
            Err(_e) => Err((StatusCode::BAD_REQUEST, Json(json!({"response" : "bad request"}))))
        }

    }

    async fn handle_create_model( Json(body): Json<Books>) -> impl IntoResponse {

        let book = Books::new(body.description);

        Json(json!({"book:" : book}))
    }


}

impl BookController {
    pub fn new() -> Self {
        BookController{}

    }
}
