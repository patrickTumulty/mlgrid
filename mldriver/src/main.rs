

use ndarray::{Array2};
use mlrust::{array_utils, ColumnVector, NeuralNetwork};

fn main() {

    test_backprop();


    run_procedural_math();
}

fn test_backprop() {
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

    let weights = vec![weights_l1, weights_l2];
    let biases = vec![biases_l1.get_data().to_owned(), biases_l2.get_data().to_owned()];

    let mut nn = NeuralNetwork::from(weights, biases);

    // for layer in nn.layers() {
    //     println!("{}", *layer);
    // }

    let result: (Vec<Array2<f32>>, Vec<Array2<f32>>) = nn.back_propagate(&input, &expected);
    for i in (0..nn.layers().len()).rev() {
        println!("{}\n{}\n{}", i + 1, result.0[i], result.1[i]);
    }
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
