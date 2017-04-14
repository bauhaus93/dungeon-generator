use std::char;

use dungeon_generator::generator::Generator;

pub fn print_dungeon(gen: &Generator) {
    let (size_x, size_y) = gen.get_size();
    println!("start dungeon draw");

    let nodes = gen.get_nodes();
    let mut scene = vec![ vec![' '; size_x]; size_y];

    for n in nodes {
        let mask = n.get_edge_mask();
        let (x, y) = n.get_pos();
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

    for y in 0..size_y {
        for x in 0..size_x {
            print!("{}", scene[y][x]);
        }
        print!("\n");
    }
    println!("dungeon size: {}x{}", size_x, size_y);
}
