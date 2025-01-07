use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_char_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let mut array_2d = Vec::new();

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Read lines from the file
    for line_result in reader.lines() {
        let line = line_result?;
        if !line.trim().is_empty() {
            array_2d.push(line.chars().collect());
        }
    }

    if array_2d.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The input file is empty or contains no valid data.",
        ))
    } else {
        Ok(array_2d)
    }
}

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
    if rows == 0 {
        return HashSet::new();
    }

    let cols = data[0].len();
    if cols == 0 {
        return HashSet::new();
    }

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

                    // Check bounds
                    if x < 0 || y < 0 || x >= rows as isize || y >= cols as isize {
                        match_found = false;
                        break;
                    }

                    // Check if the character matches the target
                    if data[x as usize][y as usize] != target_chars[k] {
                        match_found = false;
                        break;
                    }
                }

                // Add the match if all characters matched
                if match_found {
                    unique_matches.insert(((i, j), (dx, dy)));
                }
            }
        }
    }

    unique_matches
}

fn main() {
    // Get path from args
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <target_pattern>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let target = &args[2];

    // Load data into 2D array
    let data = match read_char_matrix(file_path) {
        Ok(array_2d) => array_2d,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Debug first line (optional)
    if let Some(first_line) = data.first() {
        println!("First line of data: {:?}", first_line);
    }

    // Search and count logic
    let matches = find_pattern_in_matrix(&data, target);
    println!("Found {} matches", matches.len());
}
