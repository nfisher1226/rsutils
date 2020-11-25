use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    if len > 1 {
        let mut index = 0;
        for file in args {
            if index > 0 {
                let buf = File::open(file).unwrap();
                let buf = BufReader::new(buf);
                for line in buf.lines() {
                    println!("{}", line.unwrap().chars().rev().collect::<String>());
                }
            }
            index += 1;
        }
    } else {
        loop {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            if line.is_empty() {
                break;
            }
            println!("{}", line.trim().chars().rev().collect::<String>());
        }
    }
}
