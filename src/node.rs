use std::rc::Rc;
use std::cell::RefCell;

use edge::Edge;

pub struct Node {
    id: u32,
    edge: RefCell<Vec<Rc<Edge>>>
}

fn get_id() -> u32 {
    static mut NEXT_ID: u32 = 0;

    unsafe {
        NEXT_ID += 1;
        NEXT_ID - 1
    }
}

impl Default for Node {

    fn default() -> Node {
        Node::create()
    }
}

impl Node {

    pub fn create() -> Node {
        Node {
            id: get_id(),
            edge: RefCell::new(Vec::new())
        }
    }

    pub fn add_edge(&self, e: Rc<Edge>) {
        self.edge.borrow_mut().push(e);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn is_connected_with(&self, node: &Rc<Node>) -> bool {
        for e in self.edge.borrow().iter() {
            if node.get_id() == e.get_node(0).get_id() ||
               node.get_id() == e.get_node(1).get_id() {
                   return true;
               }

        }
        false
    }

}
