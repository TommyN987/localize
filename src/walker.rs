use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn walk(path: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    visit_dirs(Path::new(path), &mut result).expect("Failed to read directory");
    result
}

fn visit_dirs(dir: &Path, result: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, result)?;
            } else if let Some(extension) = path.extension() {
                if extension == "tsx" {
                    result.push(path);
                }
            }
        }
    }
    Ok(())
}
