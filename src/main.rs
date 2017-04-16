
extern crate dungeon_generator;

mod drawer;

use std::rc::Rc;

use dungeon_generator::logger;
use dungeon_generator::graph::Graph;
use dungeon_generator::node::Node;
use dungeon_generator::node_info::NodeInfo;
use drawer::print_dungeon;

fn main() {

    match logger::init() {
        Ok(_) => {},
        Err(e) => println!("Could not init logger: {}", e)
    }

    const SIZE_X: u32 = 100;
    const SIZE_Y: u32 = 40;
    let mut nodes: Vec<Rc<Node<NodeInfo>>> = Vec::new();

    for y in 0..SIZE_Y {
        for x in 0..SIZE_X {
            nodes.push(Rc::new(Node::create(NodeInfo::new(x as i32, y as i32))));
        }
    }
    let mut graph: Graph<NodeInfo> = Graph::create_grid_with_nodes(nodes, SIZE_X, SIZE_Y);


    let tree = Graph::create_random_tree(&graph);

    print_dungeon(&tree, SIZE_X, SIZE_Y);

}
