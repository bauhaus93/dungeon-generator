use std::rc::Rc;

use node::Node;

pub struct Edge<T> {
    node: (Rc<Node<T>>, Rc<Node<T>>)
}

impl<T> Edge<T> {

    pub fn create(node_a: Rc<Node<T>>, node_b: Rc<Node<T>>) -> Edge<T> {
        Edge {
            node: (node_a, node_b)
        }
    }

    pub fn get_nodes(&self) -> (Rc<Node<T>>, Rc<Node<T>>) {
        (self.node.0.clone(), self.node.1.clone())
    }

    pub fn contains(&self, node: Rc<Node<T>>) -> bool {
        self.node.0 == node || self.node.1 == node
    }
}
