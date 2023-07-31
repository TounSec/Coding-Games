use std::fmt::format;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Node {
    x: i32,
    y: i32,
}

fn main() {
    // Number width on the X asis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x = parse_input!(input_line, i32); // the number of cells on the X axis

    // Number height on the Y asis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let y = parse_input!(input_line, i32); // the number of cells on the Y axis

    // Lines node
    let mut nodes = Vec::new();

    for i in 0..y as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim().to_string(); // width characters, each either 0 or .
        nodes.push(line);
    }

    for column in 0..y {
        for line in 0..x {
            for node in &nodes {
                if node[column][line] == '0' {
                    let info_node = format!("{} {} {} {} {} {}", x, y,
                                            check_node(true, node, line, column, y, x),
                                            check_node(false, node, line, column, y, x)
                    );

                    println!("{}", info_node);
                }
            }
        }
    }

}

fn check_node(right: bool,, x: i32, y: i32, height: i32, width: i32) -> &'static str
{
    let axe = if right { x } else { y };
    let length = if right { width } else { height };

    let count = 1;

    while (axe + count) < length {
        let node = nodes
    }

    return "test";
}