#![warn(clippy::all, clippy::pedantic)]
use getopts::Options;
use std::path::Path;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap();
    let usage = format!("Usage: {} path", progname);
    let mut opts = Options::new();
    opts.optflag("z", "zero", "Do not print trailing newline");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(m) => {
            eprintln!("Error: {}", m.to_string());
            eprintln!("{}", usage);
            process::exit(1);
        }
    };
    let path = if matches.free.len() == 1 {
        Path::new(&matches.free[0])
            .parent()
            .unwrap()
            .to_string_lossy()
    } else {
        eprintln!("{}", usage);
        process::exit(1)
    };
    if matches.opt_present("z") {
        print!("{}", path);
    } else {
        println!("{}", path);
    }
}
