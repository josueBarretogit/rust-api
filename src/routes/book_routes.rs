use axum::{routing::get, Router};

use crate::controllers::{book_controller, Controller};

use super::route::CommonRoutes;

pub struct BookRoutes {}

impl BookRoutes {
    pub fn new() -> Self {
        Self {  }
    }
}

impl CommonRoutes for BookRoutes {
    fn set_routes(&self) -> axum::Router {
        Router::new()
        .route("/books", get(book_controller::BookController::handle_get_models))
    }
}
