use std::env::current_dir;
use std::fs::create_dir;
use std::path::PathBuf;

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
