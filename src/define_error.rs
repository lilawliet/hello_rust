use std::{error::Error, fmt};

#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    InvalidFormat(String)
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::NotFound(msg) => write!(f, "File not found: {}", msg),
            FileError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            FileError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}


impl Error for FileError {}

fn read_file(path: &str) -> Result<String, FileError> {
    if path.is_empty() {
        return Err(FileError::NotFound("Path is empty".to_string()));
    }

    if path.contains("Private" ) {
        return Err(FileError::PermissionDenied("Private file".to_string()));
    }

    if path.contains("invalid") {
        return Err(FileError::InvalidFormat("Invalid file content".to_string()));
    }

    // if !path.ends_with(".txt") {
    //     return Err(FileError::InvalidFormat("Invalid file extension".to_string()));
    // }

    Ok("File content".to_string())
}


fn main() {
    match read_file("") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }

    match read_file("lib.rs" ) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }



}