// https://www.codingame.com/ide/puzzle/defibrillators

use std::io;
use std::str::FromStr;
use std::f64::consts::PI;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Input long
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let user_lon = input_line.trim().replace(",", ".").parse::<f64>().unwrap();

    // Input lat
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let user_lat = input_line.trim().replace(",", ".").parse::<f64>().unwrap();

    // Input nb defibrillator
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    // Input defibrillator description (|ID|Name|Address|Number phone|Lon|Lat|)
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
            let long = f64::from_str(parts[4].replace(",", ".").as_str()).unwrap();
            let lat = f64::from_str(parts[5].replace(",", ".").as_str()).unwrap();
            descriptions.push((id, name, address, phone, long, lat));
        }
    }

    let mut closest_defibrillator_name = String::new();
    let mut min_distance = std::f64::INFINITY;

    for (id, name, address, phone, long, lat) in &descriptions {
        let dist = calculate_distance(user_lon, user_lat, *long, *lat);

        if dist < min_distance {
            min_distance = dist;
            closest_defibrillator_name.push_str(name);
        }
    }
    println!("{}", closest_defibrillator_name);

}

fn calculate_distance(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> f64
{
    const EARTH_RADIUS: f64 = 6371.0;
    let long1_rad = lon1.to_radians();
    let lat1_rad = lat1.to_radians();
    let long2_rad = lon2.to_radians();
    let lat2_rad = lat2.to_radians();

    let d_lon = long2_rad - long1_rad;
    let d_lat = lat2_rad - lat1_rad;

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS * c

}
