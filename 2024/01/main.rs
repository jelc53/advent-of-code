use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_int_pairs(path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    // initialize two empty vectors
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    // open data file
    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    // read the file line by line
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            if let (Ok(int1), Ok(int2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                vec1.push(int1);
                vec2.push(int2);
            } else {
                eprintln!("Failed to parse integers on line: '{}'", line);
            }
        } else {
            eprintln!("Incorrect format on line '{}'", line);
        }
    }
    Ok((vec1, vec2))
}

fn compute_distance(mut vec1: Vec<i32>, mut vec2: Vec<i32>) -> i32 {
    // sort both vectors
    vec1.sort_unstable();
    vec2.sort_unstable();

    // compute distance elementwise
    vec1.iter()
        .zip(vec2.iter()) // pair elements
        .map(|(a, b)| (a - b).abs()) // abs diff
        .sum() // sum all diffs
}

fn main() {
    // specify file path
    let path = "dat.txt";

    // read data file
    match read_int_pairs(path) {
        Ok((vec1, vec2)) => {
            // compute total distance
            let dist = compute_distance(vec1, vec2);
            println!("Total dist: {}", dist);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
