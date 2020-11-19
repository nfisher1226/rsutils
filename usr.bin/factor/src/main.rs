use getopts::Options;
use primes::factors;
use std::env;
use std::io::{self};
use std::process;

static mut EXITVAL: i32 = 0;

fn factor_it(value: String) {
    let value: u64 = match value.trim().parse() {
        Ok(num) => num,
        Err(c) => {
            eprintln!("Error: {}", c);
            unsafe {
                EXITVAL = 1;
            }
            return;
        }
    };
    print!("{}:", value);
    let factors = factors(value);
    for factor in factors.iter() {
        print!(" {}", factor);
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split("/").last().unwrap();

    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}: {}", progname, f.to_string());
            process::exit(1);
        }
    };
    if !matches.free.is_empty() {
        for value in matches.free {
            factor_it(value)
        }
    } else {
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("No input");
            if line.is_empty() {
                unsafe {
                    process::exit(EXITVAL);
                }
            }
            for value in line.split_whitespace() {
                factor_it(value.to_string())
            }
        }
    }
    unsafe {
        if EXITVAL == 1 {
            process::exit(1);
        }
    }
}
