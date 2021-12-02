use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
mod solution;

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(io);
    reader.lines()
        .collect()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let lines = read_lines(File::open(file_name)?)?;
    let (horizontal, vertical) = solution::calculate_movement_location(&lines);
    println!("{}, {}", horizontal, vertical);
    let product = horizontal * vertical;
    println!("{}", product);

    println!("Calculating with aim...");
    let (horz, depth) = solution::calculate_movement_location_with_aim(&lines);
    println!("{}, {}", horz, depth);
    let aim_product = horz * depth;
    println!("{}", aim_product);

    Ok(())
}
