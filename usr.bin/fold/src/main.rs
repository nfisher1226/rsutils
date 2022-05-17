#![warn(clippy::all, clippy::pedantic)]
use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::process;
use textwrap::fill;

fn wrap_line(line: &str, args: &ArgMatches) {
    let width = match args.value_of_t("WIDTH") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    if args.is_present("WORDS") {
        let line = line.replace('\t', "    ");
        println!("{}", fill(line.trim_end(), width));
    } else if args.is_present("BYTES") {
        for (index, b) in line.as_bytes().iter().enumerate() {
            if index % width == 0 {
                println!();
            }
            print!("{}", *b as char);
        }
    } else {
        let line = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(width)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        for line in &line {
            println!("{}", line);
        }
    }
}

fn wrap_stdin(args: &ArgMatches) {
    for line in io::stdin().lock().lines() {
        wrap_line(
            &match line {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            &args,
        );
    }
}

fn wrap_file(file: &str, args: &ArgMatches) {
    let buf = File::open(file);
    let buf = match buf {
        Ok(buf) => buf,
        Err(m) => panic!("Error opening file: {:?}", m),
    };
    let buf = BufReader::new(buf);
    for line in buf.lines() {
        wrap_line(
            &match line {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            &args,
        );
    }
}

fn main() {
    let matches = Command::new("fold")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Wrap each input line to fit in specified width")
        .arg(
            Arg::new("INPUT")
                .help("The input file to use")
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("BYTES")
                .help("Count bytes rather than columns")
                .short('b')
                .long("bytes"),
        )
        .arg(
            Arg::new("WORDS")
                .help("Break at spaces")
                .short('s')
                .long("spaces"),
        )
        .arg(
            Arg::new("WIDTH")
                .help("Use width columns")
                .short('w')
                .long("width")
                .default_value("80")
                .takes_value(true),
        )
        .get_matches();
    let files: Vec<_> = match matches.values_of("INPUT") {
        Some(c) => c.collect(),
        None => vec!["-"],
    };
    for file in files {
        if file == "-" {
            wrap_stdin(&matches);
        } else {
            wrap_file(file, &matches);
        }
        if matches.is_present("BYTES") {
            println!();
        }
    }
}
