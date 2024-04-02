use axum::http::{HeaderValue, Method};
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use controllers::book_controller::BookController;
use controllers::customer_controller::CustomerController;
use controllers::Controller;
use crate::repositories::book_repository::*;
use crate::routes::*;
mod routes;
mod  models;
mod controllers;
mod repositories;




#[macro_export]
macro_rules! set_routes {
    ($controller:ty, $state:expr, $name:expr) => {
        Router::new().route($name, get(<$controller>::handle_get_models).post(<$controller>::handle_create_model))
            .with_state($state)
    };
}


#[tokio::main] 
async fn main() {


    let book_state = AppStateBooks {
        repository : BooksRepository::new()
    };


    let customer_state = AppStateCustomer {
        repository : crate::repositories::customer_repository::CustomerRepository::new()
    };


    let books  = set_routes!(BookController, book_state, "/books");
    let customer_routes = set_routes!(CustomerController, customer_state, "/customers");



    let app  = Router::new()
        .merge(books)
        .merge(customer_routes)
        .layer(

            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT])
        )
    ;



    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on 3000");

    axum::serve(listener, app).await.unwrap();
}


