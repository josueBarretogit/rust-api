use axum::BoxError;

use crate::models::book_models::Books;

use super::Repository;


#[derive(Clone)]
pub struct BooksRepository {}


impl Repository<Books> for BooksRepository {
    async fn find_all(&self) -> Result<Vec<Books>, BoxError> {
        Ok(vec![Books::new("un nuevo book".to_string()), Books::new("otro nuevo book".to_string())]) 
    } 
    
}

impl BooksRepository {

    pub fn new() -> Self {
        BooksRepository {  }
    }
    
}
