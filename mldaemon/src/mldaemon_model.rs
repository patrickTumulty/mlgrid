use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::{PathBuf};
use serde::{Deserialize, Serialize};
use graymat::column_vector::ColumnVector;
use graymat::neural_network::NeuralNetwork;
use crate::instance_manager::InstanceIdInitializer;
use crate::mldaemon_utils::{get_models_directory_path, make_dir_if_not_present};

pub struct MlDaemonModel {
    model_info: ModelInfo,
    neural_network: NeuralNetwork,
    training_data: Vec<(ColumnVector, ColumnVector)>
}

#[derive(Deserialize, Serialize)]
pub struct ModelInfo {
    name: String,
    total_test_examples: u32,
    layer_output_labels: Vec<String>
}

impl ModelInfo {
    pub fn to_file(&self, path: &PathBuf) {
        let model_info_bytes = bincode::serialize(&self).unwrap();
        let mut model_info_file = File::create(path).unwrap();
        model_info_file.write(&model_info_bytes).unwrap();
    }

    pub fn from_file(path: &PathBuf) -> ModelInfo {
        let mut model_info_bytes: Vec<u8> = Vec::new();
        let mut file = File::open(path).unwrap();
        file.read_to_end(&mut model_info_bytes).unwrap();
        return bincode::deserialize(&model_info_bytes).unwrap();
    }
}

pub const MODEL_INFO_BIN: &'static str = "model_info.bin";

impl MlDaemonModel {
    pub fn new(name: &str, neural_network: NeuralNetwork, layer_output_labels: Vec<String>, total_test_examples: u32) -> Self {
        MlDaemonModel {
            model_info: ModelInfo {
                name: name.to_string(),
                total_test_examples,
                layer_output_labels
            },
            neural_network,
            training_data: Vec::new()
        }
    }

    pub fn save(&self, save_dir: PathBuf) {
        let model_dir = save_dir.join(&self.model_info.name);
        make_dir_if_not_present(&model_dir);

        self.model_info.to_file(&model_dir.join(MODEL_INFO_BIN));

        self.neural_network.to_file(model_dir.to_str().unwrap(), "nn");
    }

    pub fn from(dir: PathBuf) -> Self {
        let model_dir_str = dir.to_str().unwrap();

        let model_info = ModelInfo::from_file(&dir.join(MODEL_INFO_BIN));

        return MlDaemonModel::new(model_info.name.as_str(),
                                  NeuralNetwork::from_file(model_dir_str, "nn"),
                                  model_info.layer_output_labels,
                                  model_info.total_test_examples);
    }

    pub fn neural_network(&self) -> &NeuralNetwork {
        &self.neural_network
    }

    pub fn neural_network_mut(&mut self) -> &mut NeuralNetwork {
        &mut self.neural_network
    }

    pub fn increment_total_test_examples(&mut self) {
        self.model_info.total_test_examples += 1;
    }

    pub fn model_info(&self) -> &ModelInfo {
        &self.model_info
    }
}

impl InstanceIdInitializer<MlDaemonModel> for MlDaemonModel {
    fn get_id(&self) -> String {
        return self.model_info.name.clone();
    }

    fn construct(instance_id: &str) -> Option<MlDaemonModel> {
        let models_dir = get_models_directory_path();
        let model_dir = models_dir.join(instance_id);
        if model_dir.exists() {
            let model = MlDaemonModel::from(models_dir.join(instance_id));
            return Some(model);
        }
        return None;
    }

    fn destruct(&self) {
        println!("Saving");
        let models_dir_path = get_models_directory_path();
        self.save(models_dir_path);
    }
}
