use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Books {
    pub description : Option<String>,
    pub title : Option<String> 
}

impl Books {
    pub fn new(description: Option<String>, title : Option<String>) -> Self {
        Books {description, title }
    }
}

