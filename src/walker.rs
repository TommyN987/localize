use std::{ffi::OsStr, path::PathBuf};

use walkdir::WalkDir;

pub fn walk(path: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.expect("Failed to read entry");
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == OsStr::new("tsx") {
                    result.push(entry.path().to_path_buf());
                }
            }
        }
    }
    result
}
