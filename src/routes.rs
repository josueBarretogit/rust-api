use axum::Router;

use crate::{models::book_models::Books, repositories::{ customer_repository::CustomerRepository, Repository}, BooksRepository};




pub mod book_routes;
pub mod customer_routes;


pub trait CommonRoutes {
    fn set_routes(&self)  -> Router;
}




#[derive(Clone)]
pub struct AppStateBooks<T : Repository<Books>>{
    pub repository :  T
}


#[derive(Clone)]
pub struct AppStateCustomer {
    pub repository : CustomerRepository
}

