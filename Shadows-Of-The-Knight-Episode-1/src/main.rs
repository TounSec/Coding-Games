use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Input width & height
    println!("Number of windows : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    println!("The building has a width of {} and a height of {}", w, h);

    // Input maximum number of turns before game over
    println!("Number of jumps : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut n = parse_input!(input_line, i32);
    println!("You have to find the bombs in {} jumps", n);

    // Start position
    println!("x, y Coordinates of starting position : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);
    println!("You start in position ({},{})", x0, y0);

    // game loop
    loop {
        if n > 0 {
            // Input direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
            println!("Direction of the bombs : ");
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let bomb_dir = input_line.trim().to_string();
            println!("The hostages are located towards {}", bomb_dir);
            n -= 1;

        } else {
            println!("😵🕹️GAME OVER🕹️😵");
            break;
        }



        // the location of the next window Batman should jump to.
        // println!("0 0");
    }
}
