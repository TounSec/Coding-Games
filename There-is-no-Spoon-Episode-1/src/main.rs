use std::env::VarError::NotPresent;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Node {
    x: i32,
    y: i32,
}

fn main() {
    // Number width on the X axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x = parse_input!(input_line, i32); // the number of cells on the X axis

    // Number height on the Y axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let y = parse_input!(input_line, i32); // the number of cells on the Y axis

    // Lines node
    let mut nodes = Vec::new();

    for i in 0..y as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim().to_string(); // width characters, each either 0 or .

        // node coordinate
        for (l, c) in line.chars().enumerate() {
            if c == '0' {
                let node = Node { x: l as i32, y: i as i32 };
                nodes.push(node);
            }
        }
    }

    for node in &nodes {
        let mut right_neighbor = Node { x: -1, y: -1 };
        let mut bottom_neighbor = Node { x: -1, y: -1 };

        // Chercher le voisin de droite
        if node.x < x - 1 {
            let neighbor_x = node.x + 1;
            if nodes.iter().find(|n| n.x == neighbor_x && n.y == node.y).is_some() {
                right_neighbor = Node { x: neighbor_x, y: node.y };
            }
        }

        // Chercher le voisin du bas
        if node.y < y - 1 {
            let neighbor_y = node.y + 1;
            if nodes.iter().find(|n| n.x == node.x && n.y == neighbor_y).is_some() {
                bottom_neighbor = Node { x: node.x, y: neighbor_y };
            }
        }

        // Formater les coordonnées et afficher le résultat
        println!(
            "{0} {1} {2} {3} {4} {5}",
            node.x, node.y, right_neighbor.x, right_neighbor.y, bottom_neighbor.x, bottom_neighbor.y
        );
    }
}
