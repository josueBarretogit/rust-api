#![forbid(unsafe_code)]
use crate::middle::*;
use crate::repositories::book_repository::*;
use crate::routes::*;
use axum::extract::DefaultBodyLimit;
use axum::http::{method, HeaderName, HeaderValue, Method};
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use controllers::book_controller::BookController;
use controllers::file_controller::*;
use controllers::Controller;
use controllers::*;
use sqlx::postgres::PgPoolOptions;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::{RequestBodyLimit, RequestBodyLimitLayer};
use tower_http::trace::TraceLayer;

mod controllers;
mod helpers;
mod middle;
mod models;
mod repositories;
mod routes;
mod services;

#[macro_export]
macro_rules! set_routes {
    ($controller:ty) => {
        Router::new().route(
            "/",
            get(<$controller>::handle_get_models).post(<$controller>::handle_create_model),
        )
    };
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not load env variables");

    let db_uri = dotenvy::var("DATABASE_URL").expect("must have a db uri env");

    let port = dotenvy::var("PORT").expect("must have a port");

    let port = port.parse::<u16>().expect("the port could not be parsed to be a valid number"); 

    let socket_address = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0 ,0, 1)), port);

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_uri)
        .await
        .expect("could not connect to database");

    let book_repo = Arc::new(db);

    let book_state = Arc::new(AppStateBooks {
        repository: BooksRepository::new(Arc::clone(&book_repo)),
    });

    //let header_middle = SetRequestHeaderLayer::if_not_present(HeaderName::from_static("myaa"),HeaderValue::from_static("my custom header"));

    let app = Router::new()
        .route(
            "/books",
            get(BookController::handle_get_models).post(BookController::handle_get_models),
        )
        .with_state(book_state)
        .route(
            "/upload",
            post(FileController::handle_upload),
        )
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        );

    let listener = tokio::net::TcpListener::bind(socket_address)
        .await
        .unwrap();

    println!("Listening on port: {}", socket_address.port());

    axum::serve(listener, app).await.unwrap();
}
