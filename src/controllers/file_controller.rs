use std::path::{Path, PathBuf};

use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use sqlx::types::chrono;

use crate::helpers::helpers::{compress_file, save_file, NewFile};

use super::FileHandler;

pub struct FileController {}

//handle errors or make middleware to allow only images to be uploaded
//make helper to make responses easier

impl FileHandler for FileController {
    async fn handle_upload(mut mult: Multipart) -> impl IntoResponse {
        let directory_to_store_files = std::env::current_dir().unwrap().join("uploads");

        while let Some(field) = mult.next_field().await.unwrap() {
            let original_file_name = field.file_name().unwrap().to_string();
            let bytes = field.bytes().await.unwrap();

            let current_time = chrono::Local::now().timestamp_millis();

            let mut new_name = PathBuf::from(current_time.to_string());

            let extension = Path::new(&original_file_name).extension().unwrap();

            new_name.set_extension(extension);

            let destination_file = directory_to_store_files.join(new_name);

            let new_file_to_store = NewFile::new(destination_file, &bytes);

            let save_file_result = save_file(new_file_to_store).await;

            match save_file_result {
                Ok(file_stored) => {
                    let compress_result =
                        compress_file(file_stored, directory_to_store_files.join("compressed"))
                            .await;

                    match compress_result {
                        Ok(_) => println!("file compressed!!"),
                        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                    }
                }
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            }
        }
        (StatusCode::CREATED, "file created".to_string())
    }

    async fn handle_delete_file(mult: Multipart) -> impl IntoResponse {
        todo!()
    }
}
