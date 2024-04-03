use axum::BoxError;
use sqlx::PgPool;

use crate::models::{book_models::Books, customer_model::Customer};

use super::Repository;


#[derive(Clone)]
pub struct CustomerRepository {
    pub db: PgPool
}


impl Repository<Customer> for CustomerRepository {
    async fn find_all(&self) -> Result<Vec<Customer>, BoxError> {

        let  db_response = sqlx::query_as::<_, Customer>("SELECT name from customers").fetch_all(&self.db).await.unwrap();


        Ok(db_response) 
    }
}

impl CustomerRepository {

    pub fn new(pool : PgPool) -> Self {
        CustomerRepository { db : pool }
    }
    
}
