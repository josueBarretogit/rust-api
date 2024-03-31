use axum::{routing::{get, post}, Router};
use controllers::{book_controller, Controller};



mod routes;
mod  models;
mod controllers;


#[tokio::main]
async fn main() {


    let app : Router<()> = Router::new()
        .route("/create", post(book_controller::BookController::create_model))
        .route("/get/:id", get(book_controller::BookController::get_by_id))
        .route("/get-books", get(book_controller::BookController::get_models));

    

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

        axum::serve(listener, app).await.unwrap();
}
