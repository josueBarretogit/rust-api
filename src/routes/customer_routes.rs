use axum::{routing::get, Router};
use super::CommonRoutes;

use crate::{controllers::{book_controller, customer_controller, Controller}, models::book_models::Books, repositories::{book_repository::{self, BooksRepository}, customer_repository::CustomerRepository, Repository}};


pub struct CustomerRoutes;

#[derive(Clone)]
pub struct AppStateCustomer {
    pub repository : CustomerRepository
}


impl CommonRoutes for CustomerRoutes {
    fn set_routes(&self)  -> Router {

        let state = AppStateCustomer {
            repository : CustomerRepository::new()
        };

        let router = Router::new() 
            .route("/customer", get(customer_controller::CustomerController::handle_get_models).post(customer_controller::CustomerController::handle_create_model))
            .with_state(state);

        router
    }
}

impl CustomerRoutes {
    pub fn new() -> Self {
        CustomerRoutes {  }
    }
}





