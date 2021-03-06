#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::process;

fn rev_file(file: &str) {
    if file == "-" {
        for line in stdin().lock().lines() {
            println!("{}", line.unwrap().trim().chars().rev().collect::<String>());
        }
    } else {
        let buf = match File::open(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("rev: {}", e);
                process::exit(1);
            }
        };
        let buf = BufReader::new(buf);
        for line in buf.lines() {
            println!("{}", line.unwrap().chars().rev().collect::<String>());
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args.push("-".to_string())
    }
    for file in args.into_iter().skip(1) {
        rev_file(&file);
    }
}
