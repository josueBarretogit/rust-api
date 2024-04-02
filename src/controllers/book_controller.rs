use axum::{  async_trait, extract::{Path, State}, response::IntoResponse, routing::get, Json};
use serde_json::{json, Value};

use crate::{models::book_models::Books, repositories::Repository, routes::book_routes::AppStateBooks};


pub struct BookController<'a> {
    repository : &'a dyn Repository<Books>
}



impl super::Controller<Books> for BookController<'_> {

    async fn handle_get_models(state: State<AppStateBooks>) -> Json<Books> {
        let book = Books::new("a new book aaaaa".to_string());
        println!("{}", state.0.repository.find_all());

        Json(book)
    }


    async fn handle_create_model( Json(body): Json<Books>) -> impl IntoResponse {

        let book = Books::new(body.description);

        Json(json!({"book:" : book}))
    }

}


impl BookController<'_> {

    pub fn new(repository : &dyn Repository<Books>) -> BookController<'_>{
        BookController { repository }
    }
    
}



