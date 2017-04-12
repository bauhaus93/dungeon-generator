
extern crate dungeon_generator;

use dungeon_generator::generator::Generator;
use dungeon_generator::logger;

fn main() {

    match logger::init() {
        Ok(_) => {},
        Err(e) => println!("Could not init logger: {}", e)
    }

    let mut gen = Generator::init(4, 4);

    println!("lelele");
}
