
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

    let graph: Graph<NodeInfo> = match Graph::create_grid(SIZE_X, SIZE_Y, &NodeInfo::new) {
        Ok(g) => g,
        Err(_) => return
    };

    graph.make_dungeon(50, 20, 10, 10, (5, 10));

    print_dungeon(&graph, SIZE_X, SIZE_Y);

}
