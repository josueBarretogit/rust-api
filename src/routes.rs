use axum::Router;

pub mod book_routes;
pub mod customer_routes;


pub trait CommonRoutes {
    fn set_routes(&self)  -> Router;
}

