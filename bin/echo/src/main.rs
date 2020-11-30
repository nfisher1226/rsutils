#![warn(clippy::all, clippy::pedantic)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    let n = len > 1 && args[1] == "-n";
    let i = if n { 2 } else { 1 };
    for (index, arg) in args.iter().enumerate().skip(i) {
        if index < len - 1 {
            print!("{} ", arg);
        } else {
            print!("{}", arg);
        }
    }
    if !n {
        println!();
    }
}
