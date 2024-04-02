use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub name : String 
}

impl Customer {
    pub fn new(name : String ) -> Customer {
        Customer {name}

    }
}
