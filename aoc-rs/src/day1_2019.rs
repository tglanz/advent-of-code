use std::env::args as get_args;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = {
        let input_path: String = get_args().skip(1).take(1).collect();
        let input_file = File::open(input_path).unwrap();
        BufReader::new(input_file)
    };

    let acc: u32 = reader.lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .map(calculate_consumption)
        .sum();

    println!("total consumption: {:#?}", acc);
}

fn calculate_consumption(mass: u32) -> u32 {
    let mut fuel = mass;
    let mut acc = 0;
    
    loop {
        fuel = (fuel / 3).checked_sub(2).unwrap_or(0);
        if fuel > 0 {
            acc += fuel;
        } else {
            break acc;
        }
    }
}
