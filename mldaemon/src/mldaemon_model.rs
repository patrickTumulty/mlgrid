use std::path::{PathBuf};
use graymat::column_vector::ColumnVector;
use graymat::neural_network::NeuralNetwork;
use crate::mldaemon_utils::make_dir_if_not_present;

pub struct MlDaemonModel {
    name: String,
    neural_network: NeuralNetwork,
    training_data: Vec<(ColumnVector, ColumnVector)>
}


impl MlDaemonModel {
    pub fn new(name: &str, neural_network: NeuralNetwork) -> Self {
        MlDaemonModel {
            name: String::from(name),
            neural_network,
            training_data: Vec::new()
        }
    }

    pub fn save(self, save_dir: PathBuf) {
        let model_dir = save_dir.join(self.name);
        make_dir_if_not_present(&model_dir);
        self.neural_network.to_file(model_dir.to_str().unwrap(), "nn");
    }

    pub fn from(dir: PathBuf) -> Self {
        let model_dir_str = dir.to_str().unwrap();
        return MlDaemonModel::new(model_dir_str,
                                  NeuralNetwork::from_file(model_dir_str, "nnn"));
    }
}
