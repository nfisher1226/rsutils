#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg};
use std::path::PathBuf;
use std::{fs, process};

fn printpath(path: PathBuf, newline: bool) {
    let path = path.into_os_string().into_string().unwrap();
    if newline {
        println!("{}", path);
    } else {
        print!("{}", path);
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
    if matches.is_present("PATH") {
        let mut newline: bool = !matches.is_present("LF");
        let paths: Vec<_> = matches.values_of("PATH").unwrap().collect();
        if paths.len() > 1 {
            if !newline {
                eprintln!("readlink: ignoring -n with multiple arguments");
            }
            newline = true;
        }
        for path in paths {
            if matches.is_present("CANON") {
                let path = fs::canonicalize(path);
                let path = match path {
                    Ok(path) => path,
                    Err(m) => {
                        eprintln!("Error: {}", m);
                        process::exit(1);
                    }
                };
                printpath(path, newline);
            } else {
                let path = fs::read_link(path);
                let path = match path {
                    Ok(path) => path,
                    Err(m) => {
                        eprintln!("Error: {}", m);
                        process::exit(1);
                    }
                };
                printpath(path, newline);
            }
        }
    } else {
        eprintln!("Error: missing operand");
        process::exit(1)
    }
}
