use std::char;
use std::rc::Rc;

use dungeon_generator::graph::Graph;
use dungeon_generator::node::Node;
use dungeon_generator::edge::Edge;
use dungeon_generator::node_info::NodeInfo;

fn get_edge_mask(node: Rc<Node<NodeInfo>>, edges: &[Rc<Edge<NodeInfo>>]) -> u8 {
    let mut mask = 0;
    for e in edges {
        let (node_a, node_b) = e.get_nodes();
        let src_pos;
        let dest_node = {
            if node == node_a {
                src_pos = node_a.get_data().get_pos();
                node_b
            }
            else if node == node_b {
                src_pos = node_b.get_data().get_pos();
                node_a
            }
            else {
                continue
            }
        };
        let dest_pos = dest_node.get_data().get_pos();

        if dest_pos.0 > src_pos.0 {
            mask |= 1;
        }
        else if dest_pos.1 > src_pos.1 {
            mask |= 2;
        }
        else if dest_pos.0 < src_pos.0 {
            mask |= 4;
        }
        else if dest_pos.1 < src_pos.1 {
            mask |= 8;
        }

        if mask == 0xF {
            break
        }
    }
    mask
}

pub fn print_dungeon(graph: &Graph<NodeInfo>, size_x: u32, size_y: u32) {

    let nodes = graph.get_nodes();
    let edges = graph.get_edges();
    let mut scene = vec![ vec![' '; size_x as usize]; size_y as usize];

    for n in nodes {
        let mask = get_edge_mask(n.clone(), &edges);
        let (x, y) = n.get_data().get_pos();
        if mask == (1 | 4) {
            scene[y as usize][x as usize] = char::from_u32(0x2501).unwrap();
        }
        else if mask == (2 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x2503).unwrap();
        }
        else if mask == (1 | 2) {
            scene[y as usize][x as usize] = char::from_u32(0x250F).unwrap();
        }
        else if mask == (2 | 4) {
            scene[y as usize][x as usize] = char::from_u32(0x2513).unwrap();
        }
        else if mask == (1 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x2517).unwrap();
        }
        else if mask == (4 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x251B).unwrap();
        }
        else if mask == (1 | 2 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x2523).unwrap();
        }
        else if mask == (2 | 4 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x252B).unwrap();
        }
        else if mask == (1 | 2 | 4) {
            scene[y as usize][x as usize] = char::from_u32(0x2533).unwrap();
        }
        else if mask == (1 | 4 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x253B).unwrap();
        }
        else if mask == (1 | 2 | 4 | 8) {
            scene[y as usize][x as usize] = char::from_u32(0x254B).unwrap();
        }
    }
    for _ in 0..size_x + 2 {
        print!("#");
    }
    print!("\n");
    for y in 0..size_y {
        print!("#");
        for x in 0..size_x {
            print!("{}", scene[y as usize][x as usize]);
        }
        print!("#\n");
    }
    for _ in 0..size_x + 2 {
        print!("#");
    }
    println!("\ndungeon size: {}x{}", size_x, size_y);
}
