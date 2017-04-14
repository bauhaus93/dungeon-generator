use std::cell::Cell;
use std::rc::Rc;

use node::Node;

pub struct Edge {
    node: [Rc<Node>; 2],
    active: Cell<bool>
}

impl Edge {

    pub fn create(node_a: Rc<Node>, node_b: Rc<Node>) -> Edge {
        Edge {
            node: [node_a, node_b],
            active: Cell::new(false),
        }
    }

    pub fn get_node(&self, index: usize) -> &Rc<Node> {
        &self.node[index]
    }

    pub fn toggle(&self) {
        self.active.set(!self.active.get())
    }

    pub fn is_active(&self) -> bool {
        self.active.get()
    }
}
