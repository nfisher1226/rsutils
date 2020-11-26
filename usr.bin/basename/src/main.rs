use getopts::Options;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap();
    let opts = Options::new();
    let matches = if let Ok(m) = opts.parse(&args[1..]) {
        m
    } else {
        eprintln!("Usage: {} string [suffix]", progname);
        process::exit(1);
    };
    if matches.free.is_empty() || matches.free.len() > 2 {
        eprintln!("Usage: {} string [suffix]", progname);
        process::exit(1);
    }
    let base = matches.free[0].split('/').last().unwrap();
    if matches.free.len() == 1 {
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
