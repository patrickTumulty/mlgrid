use mlrust::{ColumnVector, NeuralNetwork};

fn main() {

    let mut nn = NeuralNetwork::new(3, 2, vec![4, 5]);
    nn.init_network();

    let mut inputs = ColumnVector::zeros(3);
    inputs[0] = 0.8;
    inputs[1] = 0.334;
    inputs[2] = 0.78;

    let result = nn.evaluate(inputs);

    println!("{}", result);

}
