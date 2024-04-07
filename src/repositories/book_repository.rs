use axum::BoxError;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::models::book_models::Books;

use super::Repository;

#[derive(Clone, Debug)]
pub struct BooksRepository {
    pub db: PgPool,
}

impl Repository<Books> for BooksRepository {
    async fn find_all(&self) -> Result<Vec<Books>, BoxError> {
        let db_response = sqlx::query_as!(
            Books,
            "SELECT description as description, title as title from books"
        )
        .fetch_all(&self.db)
        .await?;

        Ok(db_response)
    }
}

impl BooksRepository {
    pub fn new(pool: PgPool) -> Self {
        BooksRepository { db: pool }
    }
}
