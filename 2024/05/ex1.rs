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
    // let update_values = get_valid_update_values(rules, updates);
}
