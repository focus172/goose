extern crate rand;

mod data;
mod network;

use data::Data;
use network::Network;

//#[tokio::main]
fn main() {
    let nodes = vec![7, 5, 5, 3];
    let mut network = Network::new(nodes);

    let data = Data::new();

    while let Some(data) = data.next() {
        network.train(data, 0.1);
    }

    // let test_key = "0x987654567892134";
    // let guess_range = network.make_guess(test_key);
}

