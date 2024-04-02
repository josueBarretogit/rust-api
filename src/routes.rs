use axum::Router;

use crate::{models::book_models::Books, repositories::{ customer_repository::CustomerRepository, Repository}, BooksRepository};




pub mod book_routes;
pub mod customer_routes;


pub trait CommonRoutes {
    fn set_routes(&self)  -> Router;
}




#[derive(Clone)]
pub struct AppStateBooks {
    pub repository :  BooksRepository
}




#[derive(Clone)]
pub struct AppStateCustomer {
    pub repository : CustomerRepository
}


pub struct BookRepository2 {}

impl Repository<Books> for BookRepository2 {
    async fn find_all(&self) -> Result<Vec<Books>, axum::BoxError> {
        Ok(vec![Books::new("un nuevo book".to_string()), Books::new("otro nuevo book".to_string())]) 
    }
}

impl BookRepository2 {
    pub fn new() -> Self {
        BookRepository2 {}
    }
    
}


