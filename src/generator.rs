
use rand;
use rand::distributions::{ IndependentSample, Range };

use node::Node;
use connection::Connection;

pub struct Generator<'a> {
    size: (usize, usize),
    nodes: Vec<Node<'a>>,
    edges: Vec<Connection<'a>>
}

impl<'a> Generator<'a> {

    pub fn init(size_x: usize, size_y: usize) -> Generator<'a> {
        let blocks = Array::default((size_x, size_y));

        Generator {
            size: (size_x, size_y),
            field: blocks
        }
    }

    pub fn create_random_graph(&mut self) {
    }
}
