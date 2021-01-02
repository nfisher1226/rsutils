#![warn(clippy::all, clippy::pedantic)]
use getopts::Options;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap();
    let usage = format!("Usage: {} string [suffix]", progname);
    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
            eprintln!("{}", usage);
            process::exit(1);
        }
    };
    let len = matches.free.len();
    let base = if len == 1 || len == 2 {
        matches.free[0].split('/').last().unwrap()
    } else {
        eprintln!("Usage: {} string [suffix]", progname);
        process::exit(1);
    };
    if len == 1 {
        println!("{}", base);
    } else {
        let baselen = &base.len();
        let suffixlen = matches.free[1].len();
        let split = &base.split(&matches.free[1]).next().unwrap();
        let splitlen = split.len();
        if baselen - suffixlen == splitlen {
            println!("{}", split);
        } else {
            println!("{}", base);
        }
    }
}
