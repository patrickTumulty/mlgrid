use std::fs::create_dir;
use std::path::PathBuf;

pub fn make_dir_if_not_present(models_dir: &PathBuf) {
    if !models_dir.exists() {
        create_dir(&models_dir).unwrap();
    }
}
