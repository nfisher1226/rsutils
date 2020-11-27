#![warn(clippy::all, clippy::pedantic)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut n = false;
    let mut i = 1;
    let len = args.len();
    if len > 1 && args[1] == "-n" {
        n = true;
        i = 2;
    }
    while i < len {
        if i < len - 1 {
            print!("{} ", args[i]);
        } else {
            print!("{}", args[i]);
        }
        i += 1;
    }
    if !n {
        println!();
    }
}
