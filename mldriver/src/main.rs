use ndarray::Array2;
use mlrust::{array_utils, ColumnVector, NeuralNetwork};

fn main() {

    let mut nn = NeuralNetwork::new(2, 2, vec![2]);

    println!("{}", nn);

}
