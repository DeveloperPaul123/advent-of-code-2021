use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(io);
    reader.lines()
        .collect()
}

fn calculate_movement_location_with_aim(lines: &Vec<String>) -> (u64, u64) {

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let command = parts[0].trim();
        let amount = parts[1].trim().parse::<u64>().unwrap();
        if command == "forward" {
            horizontal += amount;
            depth += aim * amount;
        }
        else if command == "up" {
            aim -= amount;
        }
        else if command == "down" {
            aim += amount;
        }
        
    }
    
    (horizontal, depth)
}

fn calculate_movement_location(lines: &Vec<String>) -> (u64, u64) {

    let mut horizontal = 0;
    let mut vertical = 0;
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let command = parts[0].trim();
        let amount = parts[1].trim().parse::<u64>().unwrap();
        if command == "forward" {
            horizontal += amount
        }
        else if command == "up" {
            vertical -= amount;
        }
        else if command == "down" {
            vertical += amount;
        }
        
    }
    
    (horizontal, vertical)

}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let lines = read_lines(File::open(file_name)?)?;
    let (horizontal, vertical) = calculate_movement_location(&lines);
    println!("{}, {}", horizontal, vertical);
    let product = horizontal * vertical;
    println!("{}", product);

    println!("Calculating with aim...");
    let (horz, depth) = calculate_movement_location_with_aim(&lines);
    println!("{}, {}", horz, depth);
    let aim_product = horz * depth;
    println!("{}", aim_product);

    Ok(())
}
