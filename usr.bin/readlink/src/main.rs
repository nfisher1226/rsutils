#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg, ArgMatches};
use std::path::PathBuf;
use std::{fs, process};

fn printpath(path: PathBuf, newline: bool) {
    print!("{}", path.into_os_string().into_string().unwrap());
    if newline {
        println!();
    }
}

fn getpath(args: &ArgMatches) {
    let paths: Vec<_> = match args.values_of("PATH") {
        Some(c) => c.collect(),
        None => {
            eprintln!("readlink: Error: missing operand");
            process::exit(1);
        }
    };
    let newline = if paths.len() > 1 {
        if args.is_present("LF") {
            eprintln!("readlink: ignoring -n with multiple arguments");
        }
        true
    } else {
        !args.is_present("LF")
    };
    for path in paths {
        if args.is_present("CANON") {
            let path = match fs::canonicalize(path) {
                Ok(path) => path,
                Err(m) => {
                    eprintln!("readlink: Error: {}", m);
                    process::exit(1);
                }
            };
            printpath(path, newline);
        } else {
            let path = match fs::read_link(path) {
                Ok(path) => path,
                Err(_) => process::exit(1),
            };
            printpath(path, newline);
        }
    }
}

fn main() {
    let matches = App::new("readlink")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Print symbolic link target or canonical file name")
        .arg(
            Arg::new("PATH")
                .about("The path to be examined")
                .multiple(true),
        )
        .arg(
            Arg::new("CANON")
                .about("Canonicalize path")
                .short('f')
                .long("canonicalize"),
        )
        .arg(
            Arg::new("LF")
                .about("Do not print the terminating newline.")
                .short('n')
                .long("no-newline"),
        )
        .get_matches();
    getpath(&matches);
}
