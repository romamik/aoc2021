use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error;

pub fn main() {

    let test_vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let input_vec = read_ints("input/day1.txt").unwrap();

    println!("test increases: {}", count_increases(&test_vec));
    println!("test windowed increases {}", count_increases_windowed(&test_vec, 3));
    println!("day1 increases: {}", count_increases(&input_vec));
    println!("day1 windowed increases {}", count_increases_windowed(&input_vec, 3));
}

fn count_increases_windowed(values: &[i32], window: usize) -> u32 {

    let mut sum_vec = Vec::new();

    for i in 0..values.len() - window + 1 {
        let window_slice = &values[i .. i + window];
        sum_vec.push(window_slice.iter().sum())
    }
    
    count_increases(&sum_vec)
}

fn count_increases(values: &[i32]) -> u32 {

    let mut prev = None;
    let mut increases = 0_u32;
    
    for v in values.iter() {

        match prev {
            Some(prev) =>
                if v > prev {
                    increases = increases + 1;
                }

            _ => ()
        }
        prev = Some(v)
    }

    increases
}

fn read_ints<P>(filename: P) -> Result<Vec::<i32>, Box<dyn error::Error>> 
where P: AsRef<Path> {

    let mut vec = Vec::new();
    let lines = read_lines(filename)?;
    for line in lines {
        vec.push(line?.parse::<i32>()?);
    }
    Ok(vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}