use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_file(file_path: &str) -> io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut is_parsing_updates = false;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            is_parsing_updates = true;
            continue;
        }

        if is_parsing_updates {
            updates.push(
                line.split(',')
                    .map(|x| x.parse::<i32>().expect("Invalid number"))
                    .collect(),
            );
        } else {
            let mut parts = line.split('|');
            rules.push((
                parts.next().unwrap().parse().expect("Invalid rule part"),
                parts.next().unwrap().parse().expect("Invalid rule part"),
            ));
        }
    }

    Ok((rules, updates))
}

fn is_update_valid(rules: &[(i32, i32)], update: &[i32]) -> bool {
    // Build a quick lookup table for positions in the update
    let positions: HashMap<i32, usize> = update
        .iter()
        .enumerate()
        .map(|(idx, &val)| (val, idx))
        .collect();

    // Validate each rule
    rules.iter().all(|&(a, b)| {
        match (positions.get(&a), positions.get(&b)) {
            (Some(&pos_a), Some(&pos_b)) => pos_a < pos_b, // Ensure `a` comes before `b`
            _ => true, // Ignore rules involving pages not in the update
        }
    })
}

fn main() {
    // Get file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Load rules and updates from the file
    let (rules, updates) = load_file(file_path).unwrap_or_else(|e| {
        eprintln!("Error loading file: {}", e);
        std::process::exit(1);
    });

    // Find middle elements of valid updates and compute the sum
    let result: i32 = updates
        .iter()
        .filter(|update| is_update_valid(&rules, update))
        .map(|update| update[update.len() / 2]) // Efficient middle-element extraction
        .sum();

    println!("Sum of middle elements: {}", result);
}
