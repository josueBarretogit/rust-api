use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Books {
    pub description : String,
    pub title : String 
}

impl Books {
    pub fn new(description: String, title : String) -> Self {
        Books {description, title }
    }
}
