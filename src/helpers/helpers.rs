use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    str::Bytes,
};

use image_compressor::{compressor::Compressor, Factor};
use sqlx::types::chrono;
use tokio::{fs::File, io::AsyncWriteExt};

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
