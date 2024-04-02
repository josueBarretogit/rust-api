use axum::{routing::get, Router};
use super::CommonRoutes;

use crate::{controllers::{book_controller, Controller}, models::book_models::Books, repositories::{book_repository::{self, BooksRepository}, Repository}};


pub struct BookRoutes;

#[derive(Clone)]
pub struct AppStateBooks {
    pub repository : BooksRepository
}


impl CommonRoutes for BookRoutes {
    fn set_routes(&self)  -> Router {
        let state = AppStateBooks {
            repository : book_repository::BooksRepository::new()
        };

        let router = Router::new() 
            .route("/books", get(book_controller::BookController::handle_get_models).post(book_controller::BookController::handle_create_model))
            .with_state(state)

        ;

        router
    }
}

impl BookRoutes {
    pub fn new() -> Self {
        BookRoutes {  }
    }
}





