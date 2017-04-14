
extern crate dungeon_generator;

mod drawer;

use dungeon_generator::generator::Generator;
use dungeon_generator::logger;
use drawer::print_dungeon;

fn main() {

    match logger::init() {
        Ok(_) => {},
        Err(e) => println!("Could not init logger: {}", e)
    }

    const SIZE_X: usize = 100;
    const SIZE_Y: usize = 100;
    let gen = Generator::init(SIZE_X, SIZE_Y);

    print_dungeon(&gen);


}
