use std::rc::Rc;
use std::collections::VecDeque;

use rand;
use rand::Rng;
use rand::distributions::{ Range, IndependentSample };

use node::Node;
use edge::Edge;

pub struct Graph<T> {
    nodes: Vec<Rc<Node<T>>>,
    edges: Vec<Rc<Edge<T>>>
}

fn random(max: u32) -> u32 {
    rand::thread_rng().gen::<u32>() % max
}

impl<T: Default> Graph<T> {

    pub fn create_grid(size_x: u32, size_y: u32, f: &Fn(i32, i32) -> T) -> Result<Graph<T>, ()> {
        info!("creating {}x{} grid", size_x, size_y);
        let mut nodes: Vec<(Rc<Node<T>>)> = Vec::new();
        let mut edges: Vec<Rc<Edge<T>>> = Vec::new();

        for y in 0..size_y {
            for x in 0..size_x {
                let node_a = Rc::new(Node::create(f(x as i32, y as i32)));
                if x > 0 {
                    let node_b = &nodes[(y * size_x + x - 1) as usize];
                    let edge = Rc::new(Edge::create(node_a.clone(), node_b.clone()));
                    node_a.add_edge(edge.clone());
                    node_b.add_edge(edge.clone());
                    edges.push(edge);
                }

                if y > 0 {
                    let node_b = & nodes[((y - 1) * size_x + x) as usize];
                    let edge = Rc::new(Edge::create(node_a.clone(), node_b.clone()));
                    node_a.add_edge(edge.clone());
                    node_b.add_edge(edge.clone());
                    edges.push(edge);
                }
                nodes.push(node_a);
            }
        }
        info!("grid generated");
        Ok(Graph {
            nodes: nodes,
            edges: edges
        })
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

    pub fn make_random_tree(&self, expansion_threshold: f32) {
        info!("creating random tree on graph");
        let node_range = Range::new(0, self.nodes.len());

        let mut tree_nodes = Vec::new();
        let mut expansion_nodes = VecDeque::new();

        expansion_nodes.push_front(self.nodes[node_range.ind_sample(&mut rand::thread_rng())].clone());

        while let Some(node) = expansion_nodes.pop_front() {
            if !tree_nodes.contains(&node) {
                tree_nodes.push(node.clone());
            }
            match node.connect_random(tree_nodes.as_slice()) {
                Some(new_node) => {
                    tree_nodes.push(new_node.clone());
                    if (tree_nodes.len() as f32) / (self.nodes.len() as f32) < expansion_threshold {
                        if random(100) < 75 {
                            expansion_nodes.push_front(new_node.clone());
                            expansion_nodes.push_back(node);
                        }
                        else {
                            expansion_nodes.push_front(new_node.clone());
                            expansion_nodes.push_front(node);
                        }
                    }



                },
                None => {}
            }
        }

        while let Some(node) = tree_nodes.pop() {
            if node.active_edges() == 1 && node.neighbour_edge_sum() != 2 {
                node.disconnect();
            }
        }
        info!("random tree generated");
    }
}
