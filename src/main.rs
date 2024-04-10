#![forbid(unsafe_code)]
use crate::middle::*;
use crate::repositories::book_repository::*;
use crate::routes::*;
use axum::extract::DefaultBodyLimit;
use axum::handler::Handler;
use axum::http::{HeaderName, HeaderValue, Method};
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use controllers::book_controller::BookController;
use controllers::file_controller::*;
use controllers::Controller;
use controllers::*;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use tower::ServiceBuilder;
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
    dotenvy::dotenv().unwrap_or_else(|err| {
        eprint!("details: {err}");
        process::exit(1)
    });

    let db_uri = dotenvy::var("DATABASE_URL").expect("must have a db uri env");

    let port = dotenvy::var("PORT").expect("must have a port");

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_uri)
        .await
        .expect("could not connect to db");

    let book_state = Arc::new(AppStateBooks {
        repository: BooksRepository::new(db.clone()),
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
            post(FileController::handle_upload).layer(middleware::from_fn(verify_images)),
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
                .allow_methods([Method::GET, Method::POST, Method::PUT]),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 3000");

    axum::serve(listener, app).await.unwrap();
}
