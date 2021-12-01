use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read_file<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let reader = BufReader::new(io);
    reader.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn count_larger_measurements(values : &Vec<i64>) -> u64 {
    let mut count = 0;
    values.iter().enumerate().for_each(|(i, _value)| {
        if i > 0 {
            if values[i] > values[i - 1] {
                count += 1;
            }
        }
    });
    return count
}

fn sliding_window_sums(values : &Vec<i64>, window_size : usize) -> Vec<i64> {
    let mut sums = Vec::new();
    for i in 0..values.len() {
        let mut local_sum = 0;
        if i + window_size > values.len() {
            break;
        }

        for win in 0..window_size {
            local_sum += values[i + win]
        }
        sums.push(local_sum)
    }
    return sums
}

fn main() -> Result<(), Error> {
    
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let numbers = read_file(File::open(file_name)?)?;
    let count = count_larger_measurements(&numbers);
    println!("{}", count);
    let windowed_sums = sliding_window_sums(&numbers, 3);
    let windowed_count = count_larger_measurements(&windowed_sums);
    println!("{}", windowed_count);
    Ok(())
}
