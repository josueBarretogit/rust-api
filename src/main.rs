#![forbid(unsafe_code)]
use std::process;
use std::sync::Arc;
use axum::handler::Handler;
use axum::http::{HeaderName, HeaderValue, Method};
use axum::routing::get;
use axum::{middleware, Extension, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use controllers::book_controller::BookController;
use controllers::customer_controller::CustomerController;
use controllers::Controller;
use tower_http::set_header::SetRequestHeaderLayer;
use tower_http::ServiceBuilderExt;
use crate::repositories::book_repository::*;
use crate::routes::*;
use crate::repositories::customer_repository::CustomerRepository;
use sqlx::postgres::PgPoolOptions;
use  crate::middle::*;


mod routes;
mod  models;
mod controllers;
mod repositories;
mod middle;
mod services;


#[macro_export]
macro_rules! set_routes {
    ($controller:ty) => {
        Router::new().route("/", get(<$controller>::handle_get_models).post(<$controller>::handle_create_model))
    };
}


#[tokio::main] 
async fn main()   {

    dotenvy::dotenv().unwrap_or_else(|err| {
        eprint!("details: {err}");
        process::exit(1)
    });

    let db_uri = dotenvy::var("DB_URI").expect("must have a db uri env");

    let  db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_uri)
        .await
        .expect("could not connect to db");

    let book_state = Arc::new(
        AppStateBooks {
            repository : BooksRepository::new(db.clone())
        });

    let header_middle = SetRequestHeaderLayer::if_not_present(HeaderName::from_static("myaa"),HeaderValue::from_static("my custom header"));


    let app  = Router::new()
        .route("/books", get(BookController::handle_get_models)).with_state(book_state)
        .layer(
            ServiceBuilder::new()
                .compression()
        )
        .layer(

            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT])
        )
        .layer(middleware::from_fn(middle::set_a_header))
    ;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on 3000");

    axum::serve(listener, app).await.unwrap();
}


