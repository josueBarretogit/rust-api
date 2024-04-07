use std::{fs , path::{Path, PathBuf}, str::Bytes};

use sqlx::types::chrono;
use tokio::{fs::File, io::AsyncWriteExt};

pub struct NewFile<'a> {
    pub file_name : String,
    pub bytes : &'a [u8],
    pub folder_to_store : &'a str
}

impl<'a> NewFile<'a> {
    pub fn new(file_name : String, bytes : &'a [u8], folder_to_store : &'a str) -> NewFile<'a> {
        NewFile {file_name, bytes, folder_to_store}
    }
}


pub async fn save_file(new_file : NewFile<'_>) -> Result<String, std::io::Error> {

    let current_dir = std::env::current_dir().unwrap().join(new_file.folder_to_store);

    if !Path::new(&current_dir).exists() {
        fs::create_dir_all(&current_dir).unwrap();
    }

    let current_time = chrono::Local::now().timestamp_millis();

    let mut new_name = PathBuf::from(current_time.to_string());

    let extension = Path::new(&new_file.file_name).extension().unwrap();

    new_name.set_extension(extension);

    let destination_file = current_dir.join(new_name.clone());


    let  creation = File::create(&destination_file).await;

    match creation {
        Ok(mut writer) => {

            writer.write_all(&new_file.bytes).await.unwrap();

        },
        Err(e) => return  Err(e),
    }


    Ok("file created".to_string())

}


pub async fn compress_file(file_to_compress : NewFile<'_>) -> Result<String, std::io::Error> {


}

