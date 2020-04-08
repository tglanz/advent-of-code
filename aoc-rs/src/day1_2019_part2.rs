use std::env::args as get_args;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() -> MyResult<()> {
    let reader = {
        let input_path = get_input_path()?;
        let input_file = File::open(&input_path)?;
        BufReader::new(input_file)
    };

    let acc = accumulate_consumptions(reader)?;
    println!("consumption: {:#?}", acc);

    Ok(())
}

fn calculate_mass_fuel_consumption(mass: u32) -> u32 {
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

fn accumulate_consumptions<R: BufRead>(reader: R) -> MyResult<u32> {
    let mut acc = 0;
    for line in reader.lines() {
        let mass = line?.parse::<u32>()?;
        acc += calculate_mass_fuel_consumption(mass);
    }

    Ok(acc)
}

fn get_input_path() -> MyResult<PathBuf> {
    let args: Vec<String> = get_args()
        .skip(1)
        .take(1)
        .collect();
    
    let input_path: PathBuf = match args.get(0) {
        Some(value) => Ok(PathBuf::from(value)),
        None => Err("missing 0 positional argument: input_path")
    }?;

    if !input_path.exists() {
        Err(format!("no such file: {:#?}", input_path))?;
    }

    Ok(input_path)
}