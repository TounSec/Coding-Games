// https://www.codingame.com/ide/puzzle/defibrillators

use std::io;
use std::str::FromStr;


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
    let user_lon = parse_input!(input_line, f64);

    // Input lat
    println!("\nSelect lat: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let user_lat = parse_input!(input_line, f64);

    // Input nb defibrillator
    println!("\nDefibrillator number: ");
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    // Input defibrillator description (|ID|Name|Address|Number phone|Lon|Lat|)
    println!("\nSelect defibrillator description: ");
    let mut descriptions = Vec::new();
    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let parts: Vec<_> = input_line.trim().split(';').collect();

        if parts.len() >= 6 {
            let id: i32 = parts[0].parse().unwrap();
            let name = parts[1].to_string();
            let address = parts[2].to_string();
            let phone = parts[3].to_string();
            let lon: f64 = f64::from_str(parts[4].replace(',', ".").as_str()).unwrap();
            let lat: f64 = f64::from_str(parts[5].replace(',', ".").as_str()).unwrap();
            descriptions.push((id, name, address, phone, lon, lat));
        }
    }



    //println!("\nInput line {}", user_lon);
    //println!("Input line {}", user_lat);
    //println!("Input line {}", n);
    //println!("Description {:?}", descriptions);

    //for (id, name, address, phone, lon, lat) in descriptions {
    //    println!("ID: {}, Name: {}, Address: {}, Phone: {}, Lon: {}, Lat: {}", id, name, address, phone, lon, lat);
    //}
    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    // println!("answer");
}
