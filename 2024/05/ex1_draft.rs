use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn load_rules_as_tuple_vec(file_path: &str) -> io::Result<Vec<(i32, i32)>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut out_vec = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            let mut parts = line.splitn(2, "|");
            let a: &str = parts.next().unwrap();
            let b: &str = parts.next().unwrap();
            out_vec.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
        } else {
            break;
        }
    }

    if out_vec.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The input file is empty or invalid.",
        ))
    } else {
        Ok(out_vec)
    }
}

fn load_updates_as_vec_of_vecs(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut out_vec = Vec::new();

    let mut skip = true;
    for line in reader.lines() {
        let line = line?;
        if skip {
            if line.trim().is_empty() {
                skip = false;
            }
            continue;
        }
        if !line.trim().is_empty() {
            let mut update_vec = Vec::new();
            for page_num in line.split(",") {
                update_vec.push(page_num.parse::<i32>().unwrap());
            }
            out_vec.push(update_vec);
        }
    }

    if out_vec.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The input file is empty or invalid.",
        ))
    } else {
        Ok(out_vec)
    }
}

fn apply_rule(update: &Vec<i32>, a: i32, b: i32) -> bool {
    // extract positions
    let pos_a = update.iter().position(|&x| x == a);
    let pos_b = update.iter().position(|&x| x == b);

    // edge case: a, b equal
    if a == b {
        return true;
    }

    // edge case: a. b not both in rule
    if pos_a.is_none() && pos_b.is_none() {
        return true;
    }

    // check if a comes before b
    match (pos_a, pos_b) {
        (Some(index_a), Some(index_b)) => index_a < index_b,
        _ => false,
    }
}

fn check_if_valid(rules: Vec<(i32, i32)>, update: Vec<i32>) -> bool {
    for (a, b) in rules {
        if !apply_rule(&update, a, b) {
            return false;
        }
    }
    true // all rules pass
}

fn middle_element(vec: &Vec<i32>) -> Option<i32> {
    if vec.len() % 2 == 1 {
        // calculate the middle index for odd-length vec
        let mid_index = vec.len() / 2;
        Some(vec[mid_index])
    } else {
        None // return none if vector length not odd
    }
}

fn get_valid_update_values(rules: Vec<(i32, i32)>, updates: Vec<Vec<i32>>) -> Vec<Option<i32>> {
    let mut out_vec = Vec::new();
    for update in updates {
        if check_if_valid(rules.clone(), update.clone()) {
            let value = middle_element(&update);
            out_vec.push(value);
        }
    }
    out_vec
}

fn main() {
    // get command line args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // load rules and updtes
    let rules = match load_rules_as_tuple_vec(file_path) {
        Ok(out_vec) => out_vec,
        Err(e) => {
            eprintln!("Error {}", e);
            std::process::exit(1);
        }
    };
    let updates = match load_updates_as_vec_of_vecs(file_path) {
        Ok(out_vec) => out_vec,
        Err(e) => {
            eprintln!("Error {}", e);
            std::process::exit(1);
        }
    };

    // check update validity
    let update_values = get_valid_update_values(rules, updates);
    println!(
        "Result q1: {}",
        update_values.iter().map(|&x| x.unwrap_or(0)).sum::<i32>()
    );
}
