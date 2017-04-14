use std::rc::Rc;

use rand;
use rand::distributions::{ IndependentSample, Range };

use node::Node;
use edge::Edge;

pub struct Generator {
    size: (usize, usize),
    nodes: Vec<Rc<Node>>,
    edges: Vec<Rc<Edge>>,
}

fn connect(node_a: Rc<Node>, node_b: Rc<Node>) -> Rc<Edge> {

    let e = Rc::new(Edge::create(node_a.clone(), node_b.clone()));

    node_a.add_edge(e.clone());
    node_b.add_edge(e.clone());

    e
}

fn create_four_neighbour_graph(size_x: usize, size_y: usize) -> (Vec<Rc<Node>>, Vec<Rc<Edge>>) {
    let mut nodes: Vec<Rc<Node>> = Vec::new();
    let mut edges: Vec<Rc<Edge>> = Vec::new();

    for y in 0..size_y {
        for x in 0..size_x {
            let node_a = Rc::new(Node::create(x as i32, y as i32));

            if x > 0 {
                let node_b = nodes[y * size_x + x - 1].clone();
                let e = connect(node_a.clone(), node_b);
                edges.push(e);
            }

            if y > 0 {
                let node_b = nodes[(y - 1) * size_x + x].clone();
                let e = connect(node_a.clone(), node_b);
                edges.push(e);
            }
            nodes.push(node_a);
        }
    }
    info!("created four neighbour graph with {} nodes and {} edges", nodes.len(), edges.len());
    (nodes, edges)
}

impl Generator {

    pub fn init(size_x: usize, size_y: usize) -> Generator {
        info!("creating new dungeon generator");

        let (nodes, edges) = create_four_neighbour_graph(size_x, size_y);

        let gen = Generator {
            size: (size_x, size_y),
            nodes: nodes,
            edges: edges,
        };
        gen
    }

    pub fn get_node_count(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn get_edge_count(&self) -> u32 {
        self.edges.len() as u32
    }

    pub fn get_nodes(&self) -> &Vec<Rc<Node>> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<Rc<Edge>> {
        &self.edges
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    fn create_graph(&mut self) {

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                self.nodes.push(Rc::new(Node::create(x as i32, y as i32)));
            }
        }

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let node_a = self.nodes[y * self.size.0 + x].clone();

                if x > 0 {
                    let node_b = self.nodes[y * self.size.0 + x - 1].clone();
                    let e = connect(node_a.clone(), node_b);
                    self.edges.push(e);
                }

                if y > 0 {
                    let node_b = self.nodes[(y - 1) * self.size.0 + x].clone();
                    let e = connect(node_a.clone(), node_b);
                    self.edges.push(e);
                }
            }
        }
    }
}
