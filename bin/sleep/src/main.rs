use std::time::Duration;
use std::{env, process, thread};

fn usage(p: &str) {
    eprintln!("Usage: {} seconds", p);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap();
    if args.len() == 2 {
        let s: u64 = match args[1].parse() {
            Ok(c) => c,
            Err(m) => {
                eprintln!("Error: {}", m);
                usage(progname);
                return;
            }
        };
        let s = Duration::new(s, 0);
        thread::sleep(s);
    } else {
        usage(progname);
    }
}
