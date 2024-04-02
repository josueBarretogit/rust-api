use std::error::Error;

use axum::{async_trait, BoxError};


pub mod book_repository;
pub mod customer_repository;


pub trait Repository<T>  {

    async fn find_all(&self) -> Result<Vec<T>, BoxError>;
    
}

