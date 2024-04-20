use std::{
    default, error::Error, fmt::{Debug, Display}, fs, num::NonZeroU16, path::{Path, PathBuf}, str::Bytes
};

use axum::{http::StatusCode, response::IntoResponse, Json};
use image_compressor::{compressor::Compressor, Factor};
use serde::Serialize;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::r#const::{BAD_REQUEST, DB_ERROR_CODE, SERVER_ERROR};

#[derive(Debug)]
pub struct NewFile<'a> {
    pub file_path: PathBuf,
    pub bytes: &'a [u8],
}

impl<'a> NewFile<'a> {
    pub fn new(file_path: PathBuf, bytes: &'a [u8]) -> NewFile<'a> {
        NewFile { file_path, bytes }
    }
}



pub enum Responder<E> 
    where E:  Debug + Display
{
    DatabaseError(E),
    BadRequest(E)
}

#[derive(Serialize, Debug)]
struct ErrorRequest {
    pub error_code : String,
    pub scope : String, 
    pub details : String
}

impl Default for ErrorRequest {
    fn default() -> Self {
        Self { 
            error_code: SERVER_ERROR.to_string(), 
            scope: SERVER_ERROR.to_string(),
            details: default::Default::default()
        }
    }
}

impl<E> IntoResponse for Responder<E> 
where E:    Debug + Display 
{
    
    fn into_response(self) -> axum::response::Response {

        let mut response = ErrorRequest::default();

        match self {
            Self::DatabaseError(error) => {
                response.details = error.to_string();
                response.error_code =   DB_ERROR_CODE.to_string();
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
            },
                Self::BadRequest(message) => {
                response.details = message.to_string();
                response.error_code = BAD_REQUEST.to_string();

                (StatusCode::BAD_REQUEST, Json(response)).into_response()
            }
        }
    }
}

pub async fn save_file(new_file: NewFile<'_>) -> Result<NewFile<'_>, std::io::Error> {
    if !Path::new(&new_file.file_path.parent().unwrap()).exists() {
        fs::create_dir_all(&new_file.file_path.parent().unwrap()).unwrap();
    }

    let creation = File::create(&new_file.file_path).await;

    match creation {
        Ok(mut writer) => {
            writer.write_all(&new_file.bytes).await.unwrap();
        }
        Err(e) => return Err(e),
    }

    Ok(NewFile::new(new_file.file_path, new_file.bytes))
}

pub async fn compress_file(
    file_to_compress: NewFile<'_>,
    directory: PathBuf,
) -> Result<(), Box<dyn Error>> {


    if !Path::new(&directory).exists() {
        fs::create_dir_all(&directory).unwrap();
    }

    let mut compressor = Compressor::new(file_to_compress.file_path, directory);

    compressor.set_factor(Factor::new(80., 0.8));
    compressor.compress_to_jpg()?;

    Ok(())
}

pub fn verify_images(content_type : &str) -> bool {

    let regex_validate_images = regex::Regex::new(r"jpg|jpeg|png").unwrap();
    regex_validate_images.is_match(content_type)
}
