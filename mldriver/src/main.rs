

use ndarray::{array};
use mlrust::{ColumnVector, NeuralNetwork};
use rand::{Rng};

fn main() {

    test();
}

fn test() {
    let mut nn = NeuralNetwork::new(2, 1, vec![2]);

    let mut test_data = Vec::with_capacity(4);
    test_data.push((ColumnVector::from(&array![[0.0, 1.0]]), ColumnVector::from(&array![[1.0]])));
    test_data.push((ColumnVector::from(&array![[1.0, 0.0]]), ColumnVector::from(&array![[1.0]])));
    test_data.push((ColumnVector::from(&array![[0.0, 0.0]]), ColumnVector::from(&array![[0.0]])));
    test_data.push((ColumnVector::from(&array![[1.0, 1.0]]), ColumnVector::from(&array![[0.0]])));

    let mut rng = rand::thread_rng();

    for _i in 0..50000 {
        let index = rng.gen_range(0..4);
        nn.train(&vec![test_data[index].clone()], 0.3);
    }

    println!("{}", nn.feed_forward(ColumnVector::from(&array![[0.0, 1.0]])));
    println!("{}", nn.feed_forward(ColumnVector::from(&array![[1.0, 0.0]])));
    println!("{}", nn.feed_forward(ColumnVector::from(&array![[0.0, 0.0]])));
    println!("{}", nn.feed_forward(ColumnVector::from(&array![[1.0, 1.0]])));
}

fn train_binary_function() {
    let mut nn = NeuralNetwork::new(4, 16, vec![10, 10]);

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
    println!("0\n{}", nn.feed_forward(test1.clone()));
    let mut test2 = ColumnVector::zeros(4);
    test2[1] = 1.0;
    test2[0] = 1.0;
    println!("3\n{}", nn.feed_forward(test2.clone()));
    let mut test3 = ColumnVector::zeros(4);
    test3[2] = 1.0;
    test3[1] = 1.0;
    test3[0] = 1.0;
    println!("7\n{}", nn.feed_forward(test3.clone()));
}


