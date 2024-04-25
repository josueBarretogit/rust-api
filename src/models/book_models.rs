use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct Books {
    pub id: i64,
    pub description: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct BooksCreateDTO {
    pub description: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct BooksUpdateDto {
    pub description: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct Roles {
    pub id: i64,
    pub rolename: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct RolesCreateDTO {
    pub rolename: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct RolesUpdateDTO {
    pub rolename: String,
}
