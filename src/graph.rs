use std::rc::Rc;

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

    pub fn make_dungeon(&self, main_len: u32, history_size: u32, history_min_ttl: u32, sub_chance: u32, sub_range: (u32, u32)) {
        info!("creating random dungeon tree on graph: main_len = {}, history_size = {}, history_min_ttl: {}, sub_chance = {}, sub_range = [{}, {}]",
               main_len, history_size, history_min_ttl, sub_chance, sub_range.0, sub_range.1);

        assert!(sub_chance <= 100);
        assert!(sub_range.1 >= sub_range.0);

        let node_range = Range::new(0, self.nodes.len());

        let mut tree_nodes = Vec::new();
        let mut expansion_front = Vec::new();
        let mut expansion_history = Vec::new();
        expansion_front.push((self.nodes[node_range.ind_sample(&mut rand::thread_rng())].clone(), main_len));

        while let Some((node, ttl)) = expansion_front.pop() {
            trace!("expansion node = {}, ttl = {}", node.get_id(), ttl);
            if ttl == 0 {
                continue;
            }
            if !tree_nodes.contains(&node) {
                tree_nodes.push(node.clone());
            }

            match node.connect_random(tree_nodes.as_slice()) {
                Some(new_node) => {
                    tree_nodes.push(new_node.clone());

                    expansion_front.push((new_node.clone(), ttl - 1));
                    expansion_history.push(new_node.clone());
                    if random(100) < sub_chance {
                        let sub_len = sub_range.0 + random(sub_range.1 - sub_range.0 + 1);
                        expansion_front.push((node.clone(), sub_len));
                        expansion_history.push(node.clone());
                    }
                    expansion_history.truncate(history_size as usize);
                },
                None => {
                    if ttl >= history_min_ttl {
                        trace!("expanding from history");
                        if let Some(old_node) = expansion_history.pop() {
                            expansion_front.push((old_node, ttl));
                        }
                    }
                }
            }
        }

        info!("random dungeon tree generated");
    }
}
