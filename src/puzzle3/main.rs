use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
mod solution;
use bitset;

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(io);
    reader.lines().collect()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let lines = read_lines(File::open(file_name)?)?;
    let gamma_string = solution::most_common_bits(&lines);
    
    println!("{}", gamma_string);

    let gamma = isize::from_str_radix(&gamma_string, 2).unwrap();
    let mut bit_set = bitset::BitSet::from_u64(gamma as u64);
    bit_set.flip_all();
    let mut bit_chars = vec!['0'; gamma_string.len()];
    for i in 0..gamma_string.len() {
        if bit_set.test(i) {
            bit_chars[i] = '1';
        } else {
            bit_chars[i] = '0';
        }
    }

    bit_chars.reverse();
    let epsilon_string: String = bit_chars.into_iter().collect();
    let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();

    let power = gamma * epsilon;

    println!("Gamma {} Epsilon {} Power {}", gamma, epsilon, power);
    Ok(())
}
