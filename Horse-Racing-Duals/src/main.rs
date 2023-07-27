use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
    The Hippodrome de Casablanca organizes a new type of horse racing : In a duel, only two horses participate in the race. For the race to be interesting, it is necessary to select two horses that have a similar power.
    Write a program that, given a set of powers, identifies the two closest powers and displays their difference with a positive integer.
 **/
fn main() {
    // Input number horse
    println!("Number 🐎 : ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    println!("{}", n);

    // Input pi horses
    let mut values_pi: Vec<i32> = Vec::new();

    for i in 0..n as usize {
        println!("Pi value 🐎 {} : ", i+1);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i32);
        values_pi.push(pi);
    }

    let mut min_dif = std::i32::MAX;
    values_pi.sort();

    for i in 0..values_pi.len()-1 {
        let dif = values_pi[i+1] - values_pi[i];

        if dif < min_dif {
            min_dif = dif
        }
    }

    println!("\n 🎯{}", min_dif);
}
