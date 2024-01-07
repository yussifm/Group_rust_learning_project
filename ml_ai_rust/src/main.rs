// We Matrix maths

use crate::lib::{network::Network, activations::SIGMOID};

pub mod lib;



//  0, 0 --> 0
//  0, 1 --> 1
//  1, 0 --> 1
//  1, 1 --> 0
fn main() {
    println!("=========== Neural matrix Started ===========");
    let inputs = vec![
        vec![0.0, 0.0], // 0,0
        vec![0.0, 1.0], // 0, 1
        vec![1.0, 0.0], // 1,0
        vec![1.0, 1.0], // 1,1
    ];
    let targets = vec![
        vec![ 0.0], // 0
        vec![ 1.0], // 1
        vec![1.0], // 1
        vec![0.0], // 0
    ];

    let mut network = Network::new_net(vec![2, 3, 1], 0.5, SIGMOID, );
    network.train(inputs, targets, 10000);

    println!("0 and 0: {:?}", network.feed_forward(vec![0.0, 0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0, 1.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0, 0.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0, 1.0]));

}
