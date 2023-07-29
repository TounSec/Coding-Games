use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct NODE {
    x: i32,
    y: i32,
}

struct COORDINATE {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
}

fn main() {
    // Number width on the X asis
    println!("Number of cells on the X axis : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let x = parse_input!(input_line, i32); // the number of cells on the X axis
    println!("Width : {}", x);

    // Number height on the Y asis
    println!("Number of cells on the Y axis : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let y = parse_input!(input_line, i32); // the number of cells on the Y axis
    println!("Height : {}", y);

    // Lines node
    let mut nodes = Vec::new();

    for i in 0..y as usize {
        println!("Width characters : ");
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim().to_string(); // width characters, each either 0 or .

        // node coordinate
        for (l, c) in line.chars().enumerate() {
            match c {
                '0'     =>      {
                    let node = NODE { x: l as i32, y: i as i32 };
                    nodes.push(format!("{} {}", node.x, node.y))
                },
                _       =>      {},
            }
        }
    }

    print!("{:?}", nodes);

    // Three coordinates: a node, its right neighbor, its bottom neighbor
    println!("0 0 1 0 0 1");
}
