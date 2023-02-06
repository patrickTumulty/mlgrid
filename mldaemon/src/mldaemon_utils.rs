use std::collections::hash_map::DefaultHasher;
use std::env::current_dir;
use std::fs::create_dir;
use std::path::PathBuf;
use std::hash::{Hash, Hasher};
use graymat::column_vector::ColumnVector;


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

pub fn ascii_gradient(f: f32) -> &'static str {
    return if f < 0.2 {
        "."
    } else if f < 0.3 {
        ":"
    } else if f < 0.4 {
        ";"
    } else if f < 0.5 {
        "+"
    } else if f < 0.6 {
        "="
    } else if f < 0.7 {
        "x"
    } else if f < 0.8 {
        "X"
    } else {
        "$"
    }
}

pub fn vec_max(v: &Vec<f32>) -> (i32, f32)  {
    let mut max: (i32, f32) = (-1, -1.0);
    for i in 0..v.len() {
        if v[i] > max.1 {
            max = (i as i32, v[i].clone());
        }
    }
    return max;
}

pub fn cvec_max(cv: &ColumnVector) -> (i32, f32)  {
    let mut max: (i32, f32) = (-1, -1.0);
    let v = cv.get_data().clone().into_raw_vec();
    for i in 0..v.len() {
        if v[i] > max.1 {
            max = (i as i32, v[i].clone());
        }
    }
    return max;
}

