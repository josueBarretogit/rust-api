use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Books {
    pub description: Option<String>,
    pub title: Option<String>,
}

impl Books {
    pub fn new(description: Option<String>, title: Option<String>) -> Self {
        Books { description, title }
    }
}


#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Roles {
    pub id : i64,
    pub rolename: String,
}



impl Roles {
    pub fn new(rolename: String ) -> Self {
        Roles { id: 1, rolename }
    }
}


