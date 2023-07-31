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
    let mut w = parse_input!(inputs[0], i32);
    let mut h = parse_input!(inputs[1], i32);
    println!("The building has a width of {} and a height of {}", w, h);

    // Input maximum number of turns before game over
    println!("\nNumber of jumps : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut n = parse_input!(input_line, i32);
    println!("You have to find the bombs in {} jumps", n);

    // Start position
    println!("\nx, y Coordinates of starting position : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);
    println!("You start in position ({},{})", x0, y0);

    // game loop
    let mut h0 = 0;
    let mut w0 = 0;
    let mut x = x0;
    let mut y = y0;

    loop {
        if n > 0 {
            // Input direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
            println!("\nDirection of the bombs : ");
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let bomb_dir = input_line.trim().to_string();
            println!("The hostages are located towards {}", bomb_dir);

            for d in bomb_dir.chars() {
                match d {
                    'U'     =>      {
                        h = y;
                        y = (y+h0) / 2;
                    },
                    'D'     =>      {
                        h0 = y;
                        y = (y+h) / 2;
                    },
                    'R'     =>      {
                        w0 = x;
                        x = (x+w) / 2;
                    },
                    'L'     =>      {
                        w = x;
                        x = (x+w0) / 2;
                    },
                    _       =>      {},
                }
            }

            println!("{} {}", x, y);
            n -= 1;

        } else {
            println!("😵🕹️GAME OVER🕹️😵");
            break;
        }



        // the location of the next window Batman should jump to.
        // println!("0 0");
    }
}
