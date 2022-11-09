use mlrust::NeuralNetwork;

fn main() {
    let a = mlrust::math_utils::sig(5.43);
    println!("{}", a);


    let mut nn = NeuralNetwork::new(3, 2, vec![4]);

    nn.init_network();

    let len = (&nn.layers()).len().to_owned();
    for i in 0..len {
        println!("{:4.4}\n", nn.layers()[i].connection_weights());
    }

}
