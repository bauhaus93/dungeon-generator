use std::cmp::PartialEq;
use std::rc::Rc;
use std::cell::RefCell;

use rand;
use rand::distributions::{ Range, IndependentSample };

use edge::Edge;

pub struct Node<T> {
    id: u32,
    edges: RefCell<Vec<Rc<Edge<T>>>>,
    data: T
}

impl<T: Default> Default for Node<T> {

    fn default() -> Node<T> {
        Node::create(T::default())
    }
}

impl<T> PartialEq for Node<T> {

    fn eq(&self, other: &Node<T>) -> bool {
        self.id == other.id
    }
}

fn get_id() -> u32 {
    static mut NEXT_ID: u32 = 0;

    unsafe {
        NEXT_ID += 1;
        NEXT_ID - 1
    }
}

impl<T> Node<T> {

    pub fn create(data: T) -> Node<T> {
        Node {
            id: get_id(),
            edges: RefCell::new(Vec::new()),
            data: data
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn add_edge(&self, edge: Rc<Edge<T>>) {
        self.edges.borrow_mut().push(edge);
    }

    pub fn active_edges(&self) -> u32 {
        self.edges.borrow()
                  .iter()
                  .fold(0, | acc, ref e | acc + match e.is_active() { true => 1, _ => 0 })
    }

    pub fn disconnect(&self) {
        for e in self.edges.borrow().iter() {
            if e.is_active() {
                e.toggle_active();
            }
        }
    }

    pub fn neighbour_edge_sum(&self) -> u32 {
        let mut sum = 0;
        for e in self.edges.borrow().iter() {
            if e.is_active() {
                sum += self.get_other(&e).active_edges();
            }
        }
        sum
    }

    pub fn connect_random(&self, possible_nodes: &[Rc<Node<T>>]) -> Option<Rc<Node<T>>> {
        let mut new_edges = Vec::new();


        for e in self.edges.borrow().iter() {
            if !e.is_active() && !possible_nodes.contains(&self.get_other(&e)) {
                new_edges.push(e.clone());
            }
        }

        if new_edges.len() == 0 {
            return None;
        }

        let range_edge = Range::new(0, new_edges.len());
        let e = new_edges[range_edge.ind_sample(&mut rand::thread_rng())].clone();
        e.toggle_active();

        Some(self.get_other(&e))
    }

    fn get_other(&self, edge: &Rc<Edge<T>>) -> Rc<Node<T>> {
        let (node_a, node_b) = edge.get_nodes();
        if self.eq(&node_a) {
            node_b
        }
        else {
            node_a
        }
    }
}
