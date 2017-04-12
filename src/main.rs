
extern crate ndarray;
extern crate rand;

mod generator;
mod node;
mod connection;

use generator::Generator;

fn main() {

    let mut gen = Generator::init(100, 100);

    gen.create_random_graph();
    println!("lelele");
}
