use axum::BoxError;
use serde::Serialize;

pub mod book_repository;
pub mod customer_repository;

pub trait Repository<T, C, U>
where
    T: Serialize,
    C: Serialize,
    U: Serialize,
{
    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error>;
    async fn insert(&self, data: C) -> Result<T, sqlx::Error>;
    async fn update(&self, id: i64, data: U) -> Result<U, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<T, sqlx::Error>;
}
