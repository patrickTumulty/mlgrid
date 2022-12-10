
use graymat::neural_network::NeuralNetwork;
use graymat::neural_network_io::{from_file, to_file};


fn main() {
    // examples::binary_to_int();
    //
    // examples::xor();


    let nn1 = NeuralNetwork::new(3, 5, vec![3, 4]);
    println!("{}", nn1);
    to_file("./test2.nn".to_owned(), nn1);

    let nn2 = from_file("./test2.nn".to_owned());
    println!("{}", nn2);

}




