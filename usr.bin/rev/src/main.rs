#![warn(clippy::all, clippy::pedantic)]
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn rev_stdin() {
    for line in stdin().lock().lines() {
        println!("{}", line.unwrap().trim().chars().rev().collect::<String>());
    }
}

fn rev_file(file: &str) {
    let buf = File::open(file).unwrap();
    let buf = BufReader::new(buf);
    for line in buf.lines() {
        println!("{}", line.unwrap().chars().rev().collect::<String>());
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args.push("-".to_string())
    }
    for file in args.into_iter().skip(1) {
        if file == "-" {
            rev_stdin();
        } else {
            rev_file(&file);
        }
    }
}
