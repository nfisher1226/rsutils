use getopts::Options;
use primes::factors;
use std::env;
use std::io::{self, BufRead};
use std::process;

fn factor_it(value: &str) -> i32 {
    let value: u64 = match value.trim().parse() {
        Ok(num) => num,
        Err(c) => {
            eprintln!("Error: {}", c);
            return 1;
        }
    };
    print!("{}:", value);
    let factors = factors(value);
    for factor in &factors {
        print!(" {}", factor);
    }
    println!();
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap();
    let mut erred = false;

    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}: {}", progname, f.to_string());
            process::exit(1);
        }
    };
    if matches.free.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            for value in line.unwrap().split_whitespace() {
                if factor_it(value) == 1 {
                    erred = true;
                }
            }
        }
        if erred {
            process::exit(1);
        }
    } else {
        for value in matches.free {
            if factor_it(&value) == 1 {
                erred = true;
            }
        }
    }
    if erred {
        process::exit(1);
    }
}
