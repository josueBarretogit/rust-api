use axum::{http::{HeaderValue, Method}, Router};
use tower_http::cors::CorsLayer;

use  crate::routes::CommonRoutes;

mod routes;
mod  models;
mod controllers;
mod repositories;



#[tokio::main] 
async fn main() {

    let book_routes = routes::book_routes::BookRoutes::new();
    let customer_routes = routes::customer_routes::CustomerRoutes::new();

    let app : Router<()> = Router::new()
        .merge(book_routes.set_routes())
        .merge(customer_routes.set_routes())
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


