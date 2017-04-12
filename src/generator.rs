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

impl Generator {

    pub fn init(size_x: usize, size_y: usize) -> Generator {

        let nodes = Vec::new();
        let edges = Vec::new();

        let mut gen = Generator {
            size: (size_x, size_y),
            nodes: nodes,
            edges: edges,
        };

        gen.create_graph();

        info!("initialized new generator instance with {} nodes and {} edges", gen.get_node_count(), gen.get_edge_count());

        gen
    }

    pub fn get_node_count(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn get_edge_count(&self) -> u32 {
        self.edges.len() as u32
    }

    fn create_graph(&mut self) {

        for _ in 0..self.size.0 * self.size.1 {
            self.nodes.push(Rc::new(Node::default()));
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
