use axum::Json;

pub mod book_repository;


pub trait Repository<T>  {

    fn find_all(&self) -> i32;
    
}

