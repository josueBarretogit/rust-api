use axum::Router;

use  crate::routes::CommonRoutes;

mod routes;
mod  models;
mod controllers;
mod repositories;



#[tokio::main]
async fn main() {

    let book_routes = routes::book_routes::BookRoutes::new();



    let app : Router<()> = Router::new()
                             .merge(book_routes.set_routes());


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on 3000");

    axum::serve(listener, app).await.unwrap();
}


