use crate::models::book_models::Books;

use super::Repository;


#[derive(Clone)]
pub struct BooksRepository {}


impl Repository<Books> for BooksRepository {
    fn find_all(&self) -> i32 {
        10
    }
}

impl BooksRepository {

    pub fn new() -> Self {
        BooksRepository {  }
    }
    
}
