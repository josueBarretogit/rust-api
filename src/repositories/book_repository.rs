use std::{sync::Arc, u32};

use axum::BoxError;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

use crate::{
    models::book_models::{Books, BooksCreateDTO, Roles, RolesCreateDTO},
    BooksUpdateDto, RolesUpdateDTO,
};

use super::Repository;

#[derive(Clone, Debug)]
pub struct BooksRepository {
    pub db: Arc<PgPool>,
}

#[derive(Clone, Debug)]
pub struct RolesRepository {
    pub db: Arc<PgPool>,
}

impl Repository<Books, BooksCreateDTO, BooksUpdateDto> for BooksRepository {
    async fn find_all(&self) -> Result<Vec<Books>, sqlx::Error> {
        let db_response = sqlx::query_as!(
            Books,
            "SELECT id as id, description as description, title as title from books"
        )
        .fetch_all(&*self.db)
        .await?;

        // let db_response = vec![Books::new(Some("a desc".to_string()), Some("a title".to_string()))];

        Ok(db_response)
    }

    async fn insert(&self, data: BooksCreateDTO) -> Result<Books, sqlx::Error> {
        todo!()
    }

    async fn delete(&self, id: i64) -> Result<Books, sqlx::Error> {
        todo!()
    }
    async fn update(&self, id: i64, data: BooksUpdateDto) -> Result<BooksUpdateDto, sqlx::Error> {
        todo!()
    }
}

impl Repository<Roles, RolesCreateDTO, RolesUpdateDTO> for RolesRepository {
    async fn find_all(&self) -> Result<Vec<Roles>, sqlx::Error> {
        let db_response =
            sqlx::query_as!(Roles, "SELECT id as id, rolename as rolename from roles")
                .fetch_all(&*self.db)
                .await?;

        Ok(db_response)
    }

    async fn insert(&self, data: RolesCreateDTO) -> Result<Roles, sqlx::Error> {
        let role_created = sqlx::query_as(
            "INSERT INTO roles (rolename) values ($1) RETURNING id ,
                                        rolename",
        )
        .bind(data.rolename)
        .fetch_one(&*self.db)
        .await?;

        Ok(role_created)
    }

    async fn delete(&self, id: i64) -> Result<Roles, sqlx::Error> {
        let role_deleted = sqlx::query_as(
            "DELETE FROM roles Where id = $1 RETURNING
                                            id, rolename",
        )
        .bind(id)
        .fetch_one(&*self.db)
        .await?;
        Ok(role_deleted)
    }

    async fn update(&self, id: i64, data: RolesUpdateDTO) -> Result<RolesUpdateDTO, sqlx::Error> {
        todo!()
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
