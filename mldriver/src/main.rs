

use ndarray::{array};

use rand::{Rng};
use graymat::{ActivationFunction, ColumnVector, examples, NeuralNetwork};


fn main() {
    examples::xor();
}


fn train_binary_function() {
    let mut nn = NeuralNetwork::new(4, 16, vec![10, 10]);
    nn.set_activation_function(ActivationFunction::RELU);

    let mut training_data: Vec<(ColumnVector, ColumnVector)> = Vec::with_capacity(5);
    for i in 0..5 {
        let mut col_in = ColumnVector::zeros(4);
        col_in[0] = (i & 0b1) as f32;
        col_in[1] = (i >> 1 & 0b1) as f32;
        col_in[2] = (i >> 2 & 0b1) as f32;
        col_in[3] = (i >> 3& 0b1) as f32;

        let mut col_out = ColumnVector::zeros(16);
        col_out[i] = 1.0;
        training_data.push((col_in, col_out));
    }

    for _i in 0..30 {
        nn.train(&training_data, 0.3);
    }

    let test1 = ColumnVector::zeros(4);
    println!("0\n{}", nn.evaluate(test1.clone()));
    let mut test2 = ColumnVector::zeros(4);
    test2[1] = 1.0;
    test2[0] = 1.0;
    println!("3\n{}", nn.evaluate(test2.clone()));
    let mut test3 = ColumnVector::zeros(4);
    test3[2] = 1.0;
    test3[1] = 1.0;
    test3[0] = 1.0;
    println!("7\n{}", nn.evaluate(test3.clone()));
}


