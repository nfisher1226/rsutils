#![warn(clippy::all, clippy::pedantic)]
use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn rev_file(file: &str) {
    let reader: Box<dyn BufRead> = if file == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        let buf = match File::open(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("rev: {}", e);
                process::exit(1);
            }
        };
        Box::new(BufReader::new(buf))
    };
    for line in reader.lines() {
        println!("{}", line.unwrap().chars().rev().collect::<String>());
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args.push("-".to_string())
    }
    args.into_iter().skip(1).for_each(|file| {
        rev_file(&file);
    });
}
