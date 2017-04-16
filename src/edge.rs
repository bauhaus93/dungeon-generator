use std::rc::Rc;
use std::cell::Cell;

use node::Node;

pub struct Edge<T> {
    node: (Rc<Node<T>>, Rc<Node<T>>),
    active: Cell<bool>
}

impl<T> Edge<T> {

    pub fn create(node_a: Rc<Node<T>>, node_b: Rc<Node<T>>) -> Edge<T> {
        Edge {
            node: (node_a, node_b),
            active: Cell::new(false)
        }
    }

    pub fn get_nodes(&self) -> (Rc<Node<T>>, Rc<Node<T>>) {
        (self.node.0.clone(), self.node.1.clone())
    }

    pub fn contains(&self, node: Rc<Node<T>>) -> bool {
        self.node.0 == node || self.node.1 == node
    }

    pub fn is_active(&self) -> bool {
        self.active.get()
    }

    pub fn toggle_active(&self) {
        self.active.set(!self.active.get());
    }
}
