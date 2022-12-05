

use ndarray::{Array2};
use mlrust::{array_utils, ColumnVector, NeuralNetwork};

fn main() {

    // back_prop_test();
    //
    train_binary_function();

    // test_backprop();
    //
    // run_procedural_math();
}

fn back_prop_test() {
    let mut input: ColumnVector = ColumnVector::zeros(2);
    input[0] = 0.1;
    input[1] = 0.5;
    let mut expected: ColumnVector = ColumnVector::zeros(2);
    expected[0] = 0.05;
    expected[1] = 0.95;

    // Layer One
    let mut weights_l1: Array2<f32> = Array2::zeros((2, 2));
    weights_l1[[0, 0]] = 0.1;
    weights_l1[[0, 1]] = 0.3;
    weights_l1[[1, 0]] = 0.2;
    weights_l1[[1, 1]] = 0.4;
    let mut biases_l1: ColumnVector = ColumnVector::zeros(2);
    biases_l1[0] = 0.25;
    biases_l1[1] = 0.25;

    // Layer Two
    let mut weights_l2: Array2<f32> = Array2::zeros((2, 2));
    weights_l2[[0, 0]] = 0.5;
    weights_l2[[0, 1]] = 0.6;
    weights_l2[[1, 0]] = 0.7;
    weights_l2[[1, 1]] = 0.8;
    let mut biases_l2: ColumnVector = ColumnVector::zeros(2);
    biases_l2[0] = 0.35;
    biases_l2[1] = 0.35;

    let mut nn = NeuralNetwork::from(vec![weights_l1, weights_l2], vec![biases_l1.get_data().clone(), biases_l2.get_data().clone()]);

    nn.back_propagate(input.get_data(), expected.get_data());

}

fn train_binary_function() {
    let mut nn = NeuralNetwork::new(4, 16, vec![10, 10]);

    let mut inputs: Vec<ColumnVector> = Vec::with_capacity(16);
    let mut outputs: Vec<ColumnVector> = Vec::with_capacity(16);
    for i in 0..16 {
        let mut col_in = ColumnVector::zeros(4);
        col_in[0] = (i & 0b1) as f32;
        col_in[1] = (i >> 1 & 0b1) as f32;
        col_in[2] = (i >> 2 & 0b1) as f32;
        col_in[3] = (i >> 3& 0b1) as f32;
        inputs.push(col_in);
        let mut col_out = ColumnVector::zeros(16);
        col_out[i] = 1.0;
        outputs.push(col_out);
    }

    for i in 0..1000 {
        nn.train(&inputs, &outputs);
    }

    let mut test1 = ColumnVector::zeros(4);
    test1[1] = 1.0;
    println!("2\n{}", nn.feed_forward(test1.clone()));
    test1[0] = 1.0;
    println!("3\n{}", nn.feed_forward(test1.clone()));
}


fn run_procedural_math() {


    let mut input: ColumnVector = ColumnVector::zeros(2);
    input[0] = 0.05;
    input[1] = 0.10;
    let mut weights_l1: Array2<f32> = Array2::zeros((2, 2));
    weights_l1[[0, 0]] = 0.15;
    weights_l1[[0, 1]] = 0.20;
    weights_l1[[1, 0]] = 0.25;
    weights_l1[[1, 1]] = 0.30;
    let mut biases_l1: ColumnVector = ColumnVector::zeros(2);
    biases_l1[0] = 0.35;
    biases_l1[1] = 0.46;


    let mut expected: ColumnVector = ColumnVector::zeros(2);
    expected[0] = 0.01;
    expected[1] = 0.99;

    let mut weights_l2: Array2<f32> = Array2::zeros((2, 2));
    weights_l2[[0, 0]] = 0.40;
    weights_l2[[0, 1]] = 0.45;
    weights_l2[[1, 0]] = 0.50;
    weights_l2[[1, 1]] = 0.55;
    let mut biases_l2: ColumnVector = ColumnVector::zeros(2);
    biases_l2[0] = 0.60;
    biases_l2[1] = 0.70;

    let activation1: Array2<f32> = input.get_data().to_owned();
    // println!("1\n{}", activation1);
    let z1: Array2<f32> = weights_l1.dot(&activation1) + biases_l1.get_data();
    // println!("{}", z1);
    let activation2 = array_utils::math::sig(&(z1.to_owned()));
    // println!("2\n{}", activation2);
    let z2: Array2<f32> = weights_l2.dot(&activation2) + biases_l2.get_data();
    // println!("{}", z2);
    let output: Array2<f32> = array_utils::math::sig(&(z2.to_owned()));
    // println!("3\n{}", output);

    let error = expected.get_data() - output.clone();
    // println!("Error 2\n{}", error);
    let error_h = weights_l2.t().dot(&error);
    // println!("Error 1\n{}", error_h);

    /*
    delta_m = lr * x * error
    delta_b = lr * error
    lr - learning rate scalar
    error - vector


    Scalar  Element wise mult         transposed col vect
    delta_m = lr * [E] * [sig_prime(output)] * [activation]^transposed
    delta_b = lr * [E] * [sig_prime(output)]
     */

    let output_prime = output.clone() * (1.0 - output.clone());
    let delta_m2 = (error.clone() * &output_prime).dot(&activation2.t());
    println!("m2\n{}", delta_m2);
    let delta_b2 = error.clone() * &output_prime;
    println!("{}", delta_b2);
    let output_h_prime = activation2.clone() * (1.0 - activation2.clone());
    let delta_m1 = (error_h.clone() * &output_h_prime).dot(&activation1.t());
    println!("m1\n{}", delta_m1);
    let delta_b2 = error_h.clone() * &output_h_prime;
    println!("{}", delta_b2);
}
