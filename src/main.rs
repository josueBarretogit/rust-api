use std::{convert::Infallible, future::Future};

use axum::{routing::{get, post}, Json, Router};
use controllers::{book_controller, Controller};
use models::book_models::Books;
use routes::{book_routes, route::CommonRoutes};

mod routes;
mod  models;
mod controllers;


#[tokio::main]
async fn main() {

    let book_routes = book_routes::BookRoutes::new();


    

    let app : Router<()> = Router::new()
        .merge(book_routes.set_routes());



    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}


fn handle_get_method() -> axum::routing::MethodRouter<(), Infallible> {
    get(|| async {
        let book = Books::new("a new book aaaaa".to_string());
        Json(book)
    })
}
