use axum::Router;

pub mod book_routes;


pub trait CommonRoutes {
    fn set_routes(&self)  -> Router;
}

