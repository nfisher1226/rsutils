use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    if len > 1 {
        for (index, file) in args.into_iter().enumerate() {
            if index > 0 {
                let buf = File::open(file).unwrap();
                let buf = BufReader::new(buf);
                for line in buf.lines() {
                    println!("{}", line.unwrap().chars().rev().collect::<String>());
                }
            }
        }
    } else {
        let stdin = stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap().trim().chars().rev().collect::<String>();
            println!("{}", line);
        }
    }
}
