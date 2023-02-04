use std::collections::hash_map::DefaultHasher;
use std::env::current_dir;
use std::fs::create_dir;
use std::path::PathBuf;
use std::hash::{Hash, Hasher};


pub fn make_dir_if_not_present(models_dir: &PathBuf) {
    if !models_dir.exists() {
        create_dir(&models_dir).unwrap();
    }
}

pub fn get_models_directory_path() -> PathBuf {
    let current_dir: PathBuf = current_dir().unwrap();
    let models_dir: PathBuf = current_dir.join("models");
    models_dir
}

pub fn hash_string(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:X}", hasher.finish())
}
