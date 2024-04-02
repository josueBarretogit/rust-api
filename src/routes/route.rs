use axum::{ routing::get, Json, Router};
use serde::Serialize;

use crate::{controllers::Controller, models::book_models::Books};


pub trait CommonRoutes {
    fn set_routes(&self) -> Router;

}

pub struct BookRoutes {

}


impl CommonRoutes for BookRoutes {
    fn set_routes(&self) -> Router {
    }
}

    




