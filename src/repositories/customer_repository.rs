use axum::BoxError;
use sqlx::PgPool;

use crate::models::{book_models::Books, customer_model::Customer};

use super::Repository;

#[derive(Clone)]
pub struct CustomerRepository {
    pub db: PgPool,
}
