use std::fs;

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
