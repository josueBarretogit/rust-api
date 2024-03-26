use std::{fs::File, io::{self, Read}};

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn read_file(path : String) -> io::Result<File> {
    let open_file = File::open(path);
    open_file
}

pub fn read_username() -> Result<String, io::Error> {
    let mut usernamer = String::new();

    File::open("hello.txt")?.read_to_string(&mut usernamer)?;

    Ok(usernamer)
} 
