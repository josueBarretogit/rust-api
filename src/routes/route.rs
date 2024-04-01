use axum::{ routing::get, Json, Router};
use serde::Serialize;

use crate::controllers::{ Controller};


pub trait CommonRoutes {
    fn set_routes(&self) -> Router;
}

pub struct RouterModel {
    pub model : String,
    router : Router,
}


    




