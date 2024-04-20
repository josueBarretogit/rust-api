use std::sync::Arc;

use axum::BoxError;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::models::book_models::{Books, Roles};

use super::Repository;

#[derive(Clone, Debug)]
pub struct BooksRepository {
    pub db: Arc<PgPool>,
}


#[derive(Clone, Debug)]
pub struct RolesRepository {
    pub db: Arc<PgPool>,
}




impl Repository<Books> for BooksRepository {
    async fn find_all(&self) -> Result<Vec<Books>, sqlx::Error> {

        let db_response = sqlx::query_as!(
            Books,
            "SELECT description as description, title as title from books"
        )
        .fetch_all(&*self.db)
        .await?;

        // let db_response = vec![Books::new(Some("a desc".to_string()), Some("a title".to_string()))];

        Ok(db_response)
    }

    async fn insert(&self, data: Books) -> Result<Books, sqlx::Error> {
        todo!()
    }
    
}

impl Repository<Roles> for RolesRepository {
    
    async fn find_all(&self) -> Result<Vec<Roles>, sqlx::Error> {
        todo!()
    }
    async fn insert(&self, data: Roles) -> Result<Roles, sqlx::Error> {

        let db_response =  sqlx::query("INSERT INTO roles (rolename) values (?) RETURNING id , rolename").bind(data.rolename).execute(&*self.db).await?;

        Ok(Roles { id : 1, rolename : "nuevo".into()})

        
    }
}

impl RolesRepository {
    
    pub fn new(pool: Arc<PgPool>) -> Self {
        RolesRepository { db: pool }
    }
}

impl BooksRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        BooksRepository { db: pool }
    }

}
