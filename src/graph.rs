use std::rc::Rc;
use std::collections::VecDeque;
use std::iter::FromIterator;

use rand;
use rand::Rng;

use node::Node;
use edge::Edge;

pub struct Graph<T> {
    nodes: Vec<Rc<Node<T>>>,
    edges: Vec<Rc<Edge<T>>>
}

impl<T: Default> Graph<T> {

    pub fn create_grid(size_x: u32, size_y: u32) -> Graph<T> {
        let mut nodes: Vec<Rc<Node<T>>> = Vec::new();
        let mut edges: Vec<Rc<Edge<T>>> = Vec::new();

        for y in 0..size_y {
            for x in 0..size_x {
                let node_a = Rc::new(Node::default());

                if x > 0 {
                    let node_b = nodes[(y * size_x + x - 1) as usize].clone();
                    let e = Rc::new(Edge::create(node_a.clone(), node_b));
                    edges.push(e);
                }

                if y > 0 {
                    let node_b = nodes[((y - 1) * size_x + x) as usize].clone();
                    let e = Rc::new(Edge::create(node_a.clone(), node_b));
                    edges.push(e);
                }
                nodes.push(node_a);
            }
        }

        Graph {
            nodes: nodes,
            edges: edges
        }
    }

    pub fn create_grid_with_nodes(nodes: Vec<Rc<Node<T>>>, size_x: u32, size_y: u32) -> Graph<T> {
        let mut edges: Vec<Rc<Edge<T>>> = Vec::new();

        for y in 0..size_y {
            for x in 0..size_x {
                let node_a = nodes[(y * size_x + x) as usize].clone();

                if x > 0 {
                    let node_b = nodes[(y * size_x + x - 1) as usize].clone();
                    let e = Rc::new(Edge::create(node_a.clone(), node_b));
                    edges.push(e);
                }

                if y > 0 {
                    let node_b = nodes[((y - 1) * size_x + x) as usize].clone();
                    let e = Rc::new(Edge::create(node_a, node_b));
                    edges.push(e);
                }
            }
        }

        Graph {
            nodes: nodes,
            edges: edges
        }
    }

    pub fn create_empty() -> Graph<T> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    pub fn create(nodes: &[Rc<Node<T>>], edges: &[Rc<Edge<T>>]) -> Graph<T> {
        Graph {
            nodes: nodes.to_vec(),
            edges: edges.to_vec()
        }
    }

    pub fn add_node(&mut self, node: Rc<Node<T>>) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Rc<Edge<T>>) {
        self.edges.push(edge);
    }

    pub fn get_nodes(&self) -> &[Rc<Node<T>>] {
        &self.nodes
    }

    pub fn get_edges(&self) -> &[Rc<Edge<T>>] {
        &self.edges
    }

    pub fn create_random_tree(&self) -> Graph<T> {
        let mut nodes = Vec::new();
        let mut tree_edges = Vec::new();

        let mut shuffled_edges = Vec::new();
        for e in &self.edges {
            shuffled_edges.push(e.clone());
        }
        rand::thread_rng().shuffle(&mut shuffled_edges);

        let mut edges = VecDeque::new();
        for e in shuffled_edges {
            edges.push_front(e.clone());
        }

        while let Some(edge) = edges.pop_front() {
            let (node_a, node_b) = edge.get_nodes();
            let contains_a = nodes.contains(&node_a);
            let contains_b = nodes.contains(&node_b);
            let contains_both = contains_a && contains_b;

            if (!contains_both) && (contains_a || contains_b) {
                    if contains_a {
                        nodes.push(node_b);
                    }
                    else if contains_b {
                        nodes.push(node_a);
                    }
                    tree_edges.push(edge);
            }
            else if nodes.len() == 0 {
                nodes.push(node_a);
                nodes.push(node_b);
                tree_edges.push(edge);
            }
            else if !contains_both {
                edges.push_back(edge);
            }
        }


        nodes.retain( | ref node | {
            let mut count = 0;
            let node = node.clone();

            for e in &tree_edges {
                let nodes = e.get_nodes();
                if node.eq(&nodes.0) || node.eq(&nodes.1) {
                    count += 1;
                }
                if count > 1 {
                    return true;
                }
            }
            false
        } );


        Graph::create(nodes.as_slice(), tree_edges.as_slice())
    }
}
