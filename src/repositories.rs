use axum::BoxError;

pub mod book_repository;
pub mod customer_repository;

pub trait Repository<T> {
    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error>;
    async fn insert(&self, data: T) -> Result<T, sqlx::Error>;
}
