use std::collections::BTreeSet;
use std::collections::HashSet;
use std::convert::TryInto;
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

fn unique_sorted_elements(vec: Vec<i32>) -> Vec<i32> {
    let set: BTreeSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

fn unique_elements(vec: Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

fn count_occurences(vec: &Vec<i32>, x: i32) -> usize {
    vec.iter().filter(|&&value| value == x).count()
}

fn compute_similarity(mut vec1: Vec<i32>, mut vec2: Vec<i32>) -> i32 {
    // get unique list of elements
    let vec1_unique = unique_elements(vec1);

    // compute similarity
    let mut sim_vec = Vec::new();
    for ele in vec1_unique {
        let n: usize = count_occurences(&vec2, ele);
        let n_int: i32 = n.try_into().expect("Cannot convert to int!");
        sim_vec.push(n_int * ele);
    }

    // sum over similarity scores
    sim_vec.into_iter().sum()
}

fn compute_distance(mut vec1: Vec<i32>, mut vec2: Vec<i32>) -> i32 {
    // sort both vectors
    vec1.sort_unstable();
    vec2.sort_unstable();

    // compute distance
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
            let dist = compute_distance(vec1.clone(), vec2.clone());
            println!("Total dist: {}", dist);

            // compute simialrity
            let sim = compute_similarity(vec1.clone(), vec2.clone());
            println!("Similarity score: {}", sim);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
