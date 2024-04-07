use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};

use crate::helpers::helpers::{save_file, NewFile};

use super::FileHandler;

pub struct FileController {}

impl FileHandler for FileController {
    async fn handle_upload(mut mult: Multipart) -> impl IntoResponse {

        while let Some(field) = mult.next_field().await.unwrap() {
            let original_file_name = field.file_name().unwrap().to_string();
            let bytes = field.bytes().await.unwrap();

            let new_file_to_store = NewFile::new(original_file_name, &bytes, "uploads3");
            

            let save_file_result = save_file(new_file_to_store).await; 

            match save_file_result {
                Ok(_) => {},
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR,  e.to_string())
            }

        }
        (StatusCode::CREATED,  "file created".to_string())
    }

    async fn handle_delete_file(mult: Multipart) -> impl IntoResponse {
        todo!()
    }
}
