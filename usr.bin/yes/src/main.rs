#![warn(clippy::all, clippy::pedantic)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = if args.len() > 1 {
        args[1].to_string()
    } else {
        String::from("y")
    };
    loop {
        println!("{}", s);
    }
}
