use std::rc::Rc;
use std::cell::RefCell;

use edge::Edge;

pub struct Node {
    id: u32,
    pos: (i32, i32),
    edge: RefCell<Vec<Rc<Edge>>>
}

pub enum Direction {
    PLUS_X, MINUS_X, PLUS_Y, MINUS_Y
}

fn get_id() -> u32 {
    static mut NEXT_ID: u32 = 0;

    unsafe {
        NEXT_ID += 1;
        NEXT_ID - 1
    }
}

impl Node {

    pub fn create(pos_x: i32, pos_y: i32) -> Node {
        Node {
            id: get_id(),
            pos: (pos_x, pos_y),
            edge: RefCell::new(Vec::new())
        }
    }

    pub fn add_edge(&self, e: Rc<Edge>) {
        self.edge.borrow_mut().push(e);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
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

    pub fn get_edge_mask(&self) -> u8 {
        let mut mask = 0;
        for e in self.edge.borrow().iter() {
            if e.is_active() {
                match self.get_direction(e) {
                    Direction::PLUS_X => mask |= 1,
                    Direction::PLUS_Y => mask |= 2,
                    Direction::MINUS_X => mask |= 4,
                    Direction::MINUS_Y => mask |= 8
                }
            }
        }
        mask
    }

    pub fn get_direction(&self, edge: &Rc<Edge>) -> Direction {
        let (src, dest) =
        if self.id == edge.get_node(0).get_id() {
            (edge.get_node(0), edge.get_node(1))
        }
        else if self.id == edge.get_node(1).get_id() {
            (edge.get_node(1), edge.get_node(0))
        }
        else {
            unreachable!();
        };

        let (src_x, src_y) = src.get_pos();
        let (dest_x, dest_y) = dest.get_pos();

        if src_x < dest_x {
            Direction::PLUS_X
        }
        else if src_x > dest_x {
            Direction::MINUS_X
        }
        else if src_y < dest_y {
            Direction::PLUS_Y
        }
        else if src_y > dest_y {
            Direction::MINUS_Y
        }
        else {
            unreachable!();
        }
    }

}
