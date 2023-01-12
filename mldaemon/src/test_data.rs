use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TestData {
    pub label: String,
    pub data: Vec<f32>,
    pub target: Vec<f32>
}


impl TestData {
    pub fn to_file(self, path: String) {
        let mut file = File::create(&path).unwrap();
        let bytes = bincode::serialize(&self).unwrap();
        file.write(&bytes).unwrap();
    }

    pub fn from_file(path: String) -> Self {
        let mut file = File::open(&path).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let data: TestData = bincode::deserialize(&bytes).unwrap();
        return data;
    }

}
