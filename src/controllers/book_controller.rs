use axum::middleware::Next;
use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};
use axum::{extract::*, BoxError};
use serde_json::{json, Value};
use sqlx::types::chrono;
use core::panic;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::ExtractJwt;
use crate::{models::book_models::Books, repositories::Repository, AppStateBooks, BooksRepository};

pub struct BookController {}

impl super::Controller<Books, AppStateBooks<BooksRepository>> for BookController {
    async fn handle_get_models(
        state: State<Arc<AppStateBooks<BooksRepository>>>,
    ) -> Result<Json<Vec<Books>>, (StatusCode, Json<Value>)> {
        let repository = &state.repository;

        let response = repository.find_all().await;

        match response {
            Ok(books) => Ok(Json(books)),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"response" : "bad request", "details" : e.to_string()})),
            )),
        }
    }

    async fn handle_create_model(Json(body): Json<Books>) -> impl IntoResponse {
        let book = Books::new(body.description, body.title);

        Json(json!({"book:" : book}))
    }
}


impl BookController {
    pub async fn tes(ExtractJwt(jwt): ExtractJwt) -> impl IntoResponse {
        println!("the jwt is: {}", jwt.to_str().unwrap());

        "hola middle"
    }

    pub async fn upload_images(mut mult: Multipart) -> impl IntoResponse {

        let  current_dir = std::env::current_dir().unwrap().join("uploads");

        if !Path::new(&current_dir).exists() {
            fs::create_dir_all(&current_dir).unwrap();
        }
            

        while let Some(field) = mult.next_field().await.unwrap() {
            let original_file_name = field.file_name().unwrap().to_string();
            let data = field.bytes().await.unwrap();

            println!("original file name : {:?}", original_file_name);

            let current_time = chrono::Local::now().timestamp_millis();

            let mut file_name = PathBuf::from(current_time.to_string());

            let extension  = Path::new(&original_file_name).extension().unwrap();

            file_name.set_extension(extension);

            let destination_file  = current_dir.join(file_name);


            println!("{:#?}", destination_file);

            let mut newfile = File::create(&destination_file).await.unwrap_or_else(|err| {
                println!("{err}");
                panic!("could not ")
            });

            newfile.write_all(&data).await.unwrap();
        }
        "file created"
    }
}
