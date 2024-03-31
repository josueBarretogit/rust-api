use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Books {
    pub description : String 
}

impl Books {
    pub fn new(description: String) -> Self {
        Books {description }
    }
}
