use axum::BoxError;

pub mod book_repository;
pub mod customer_repository;

pub trait Repository<T, CREATEDTO> {
    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error>;
    async fn insert(&self, data: CREATEDTO) -> Result<T, sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<T, sqlx::Error>;
}
