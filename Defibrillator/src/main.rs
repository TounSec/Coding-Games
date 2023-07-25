// https://www.codingame.com/ide/puzzle/defibrillators

use std::io;


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Input long
    println!("Select long: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lon = input_line.trim().to_string();

    // Input lat
    println!("\nSelect lat: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = input_line.trim().to_string();

    // Input nb defibrillator
    println!("\nDefibrillator number: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    // Input defibrillator description
    println!("\nSelect defibrillator description: ");
    let mut description = Vec::new();
    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        description.push(input_line.trim().to_string())
    }

    println!("Input line {}", lon);
    println!("Input line {}", lat);
    println!("Input line {}", n);
    println!("Input line {:?}", description);



    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    // println!("answer");
}
