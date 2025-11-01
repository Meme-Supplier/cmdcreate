use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::utils::msgs::error;

pub fn read_file_to_string(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            error("Error reading file:", &format!("\"{file_path}\": {e}"));
            String::new()
        }
    }
}

pub fn write_to_file(path: &str, contents: &str) {
    let mut file = File::create(path).expect("Failed to create the file");

    file.write_all(contents.as_bytes())
        .expect("Failed to write to file");
}

// Not used right now
pub fn _folder_exists(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn _create_folder(path: &str) {
    match fs::create_dir_all(path) {
        Ok(_) => {}
        Err(e) => error(&format!("Failed to create folder: {path}"), &e.to_string()),
    }
}

pub fn _create_file(path: &str) {
    if let Some(parent) = Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _ = fs::File::create(path);
}

pub fn delete_file(path: &str) {
    if let Err(e) = fs::remove_file(path) {
        error(&format!("Failed to delete file {path}:"), &e.to_string());
    }
}

pub fn delete_folder(path: &str) {
    if let Err(e) = fs::remove_dir_all(path) {
        error(&format!("Failed to delete folder {path}:"), &e.to_string());
    }
}
