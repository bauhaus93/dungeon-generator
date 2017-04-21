
extern crate dungeon_generator;

use dungeon_generator::logger;
use dungeon_generator::graph::Graph;
use dungeon_generator::node_info::NodeInfo;
use dungeon_generator::drawer::print_dungeon;

fn main() {

    match logger::init() {
        Ok(_) => {},
        Err(e) => println!("Could not init logger: {}", e)
    }

    const SIZE_X: u32 = 80;
    const SIZE_Y: u32 = 40;
    const MAIN_LEN: u32 = 50;
    const HISTORY_SIZE: u32 = 20;
    const HISTORY_MIN_TTL: u32 = 10;
    const SUB_CHANCE: u32 = 10;
    const SUB_MIN: u32 = 5;
    const SUB_MAX: u32 = 10;


    let graph: Graph<NodeInfo> = match Graph::create_grid(SIZE_X, SIZE_Y, &NodeInfo::new) {
        Ok(g) => g,
        Err(_) => return
    };

    graph.make_dungeon( MAIN_LEN,
                        HISTORY_SIZE,
                        HISTORY_MIN_TTL,
                        SUB_CHANCE,
                        (SUB_MIN, SUB_MAX));

    print_dungeon(&graph, SIZE_X, SIZE_Y);

}
