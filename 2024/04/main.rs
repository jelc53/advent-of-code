use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

/// Reads the character matrix from the file and returns a 2D vector of characters.
fn read_char_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            matrix.push(line.chars().collect());
        }
    }

    if matrix.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The input file is empty or invalid.",
        ))
    } else {
        Ok(matrix)
    }
}

/// Finds occurrences of a target pattern in the matrix in all 8 directions.
fn find_pattern_in_matrix(
    data: &Vec<Vec<char>>,
    target: &str,
) -> HashSet<((usize, usize), (isize, isize))> {
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (-1, 1),  // diagonal up-right
        (-1, -1), // diagonal up-left
    ];

    let rows = data.len();
    let cols = data[0].len();
    let target_chars: Vec<char> = target.chars().collect();
    let target_len = target_chars.len();
    let mut unique_matches = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut match_found = true;

                for k in 0..target_len {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;

                    if x < 0
                        || y < 0
                        || x >= rows as isize
                        || y >= cols as isize
                        || data[x as usize][y as usize] != target_chars[k]
                    {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    unique_matches.insert(((i, j), (dx, dy)));
                }
            }
        }
    }

    unique_matches
}

/// Finds X-MAS patterns (two MAS sequences forming an "X") in the matrix.
fn find_xmas_in_matrix(data: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let rows = data.len();
    let cols = data[0].len();
    let mut unique_matches = HashSet::new();

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if data[i][j] == 'A' {
                let d1 = data[i - 1][j - 1];
                let d2 = data[i + 1][j + 1];
                let o1 = data[i + 1][j - 1];
                let o2 = data[i - 1][j + 1];

                if ((d1 == 'M' && d2 == 'S') || (d1 == 'S' && d2 == 'M'))
                    && ((o1 == 'M' && o2 == 'S') || (o1 == 'S' && o2 == 'M'))
                {
                    unique_matches.insert((i, j));
                }
            }
        }
    }

    unique_matches
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <target_pattern>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let target = &args[2];

    // Load the matrix from the file
    let data = match read_char_matrix(file_path) {
        Ok(matrix) => matrix,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Print the first line of the matrix (optional debug step)
    if let Some(first_line) = data.first() {
        println!("First line of data: {:?}", first_line);
    }

    // Find and count target pattern matches
    let matches = find_pattern_in_matrix(&data, target);
    println!(
        "Found {} matches for target pattern '{}'",
        matches.len(),
        target
    );

    // Find and count X-MAS patterns
    let xmas_matches = find_xmas_in_matrix(&data);
    println!("Found {} X-MAS matches", xmas_matches.len());
}
