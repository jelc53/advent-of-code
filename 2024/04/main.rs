use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_char_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let mut array_2d = Vec::new();

    // open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // read lines from file
    for line_result in reader.lines() {
        let line = line_result?;
        array_2d.push(line.chars().collect());
    }

    Ok(array_2d)
}

fn find_pattern_in_matrix(
    data: &Vec<Vec<char>>,
    target: &str,
) -> HashSet<((usize, usize), (isize, isize))> {
    let directions = [
        (0, 1), // right
        // (0, -1),  // left
        (1, 0), // down
        // (-1, 0),  // up
        (1, 1), // down-right
        // (-1, -1), // up-left
        (-1, 1), // up-right
                 // (1, -1),  // down-left
    ];

    let rows = data.len();
    let cols = data[0].len();
    let target_len = target.len();
    let mut unique_matches = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in directions.iter() {
                let mut found_forward = true;

                for k in 0..target_len {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;

                    // check bounds
                    if x < 0 || y < 0 || x >= rows as isize || y >= cols as isize {
                        found_forward = false;
                        break;
                    }

                    // check forward match
                    if data[x as usize][y as usize] != target.chars().nth(k).unwrap() {
                        found_forward = false;
                        break;
                    }
                }

                if found_forward {
                    unique_matches.insert(((i, j), (dx, dy)));
                }
            }
        }
    }

    unique_matches
}

fn main() {
    // get path from args
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <target_pattern>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let target = &args[2];

    // load data into 2d array
    let result = read_char_matrix(file_path);
    let data = match result {
        Ok(array_2d) => array_2d,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(first_line) = data.first() {
        println!("{:?}", first_line);
    }

    // search and count logic
    let matches = find_pattern_in_matrix(&data, target);
    println!("Found {} matches", matches.len());
}
