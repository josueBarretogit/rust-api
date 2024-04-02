use axum::BoxError;

use crate::models::{book_models::Books, customer_model::Customer};

use super::Repository;


#[derive(Clone)]
pub struct CustomerRepository {}


impl Repository<Customer> for CustomerRepository {
    async fn find_all(&self) -> Result<Vec<Customer>, BoxError> {
        return Err("aaaa".into());
        Ok(vec![Customer::new("asdalkj".to_string())])
    }
}

impl CustomerRepository {

    pub fn new() -> Self {
        CustomerRepository {  }
    }
    
}
