#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg, ArgMatches};
use std::io::{stdin, Read};
use std::{fs, process};

struct Values {
    name: String,
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    max: usize,
}

impl Values {
    fn print_values(&self, flags: &[char]) {
        let mut line = String::new();
        for flag in flags {
            match flag {
                'l' => line = format!("{}\t{}", line, self.lines),
                'w' => line = format!("{}\t{}", line, self.words),
                'm' => line = format!("{}\t{}", line, self.chars),
                'c' => line = format!("{}\t{}", line, self.bytes),
                'L' => line = format!("{}\t{}", line, self.max),
                _ => {
                    eprintln!("Illegal input");
                    process::exit(1);
                }
            }
        }
        if self.name != "-" {
            line = format!("{}\t{}", line, self.name);
        }
        println!("{}", line);
    }
}

fn get_flags(args: &ArgMatches) -> Vec<char> {
    let mut flags = Vec::new();
    if args.is_present("LINES") {
        flags.push('l');
    }
    if args.is_present("WORDS") {
        flags.push('w');
    }
    if args.is_present("CHARS") {
        flags.push('m');
    }
    if args.is_present("BYTES") {
        flags.push('c');
    }
    if args.is_present("MAX") {
        flags.push('L');
    }
    if flags.is_empty() {
        flags.push('c');
        flags.push('l');
        flags.push('w');
    }
    flags
}

fn get_values(file: &str, totals: &mut Values, flags: &[char]) {
    let mut contents = String::new();
    if file == "-" {
        stdin().read_to_string(&mut contents).unwrap();
    } else {
        contents = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("wc: {}", e);
                process::exit(1);
            }
        };
    }
    let mut f = Values {
        name: (*file).to_string(),
        lines: 0,
        words: 0,
        chars: 0,
        bytes: 0,
        max: 0,
    };
    for flag in flags {
        match flag {
            'l' => {
                f.lines = contents.lines().count();
                totals.lines += f.lines;
            }
            'w' => {
                f.words = contents.split_whitespace().count();
                totals.words += f.words;
            }
            'm' => {
                f.chars = contents.chars().count();
                totals.chars += f.words;
            }
            'c' => {
                f.bytes = contents.bytes().count();
                totals.bytes += f.bytes;
            }
            'L' => {
                f.max = 0;
                for line in contents.lines() {
                    let max = line.chars().count();
                    if max > f.max {
                        f.max = max;
                    }
                }
                totals.max += f.max;
            }
            _ => panic!("Illegal input"),
        };
    }
    f.print_values(&flags);
}

fn main() {
    let matches = App::new("wc")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Print newline, word, and byte counts for each file")
        .arg(
            Arg::new("INPUT")
                .about("The input file to use")
                .multiple(true),
        )
        .arg(
            Arg::new("BYTES")
                .about("Print the byte counts")
                .short('c')
                .long("bytes"),
        )
        .arg(
            Arg::new("CHARS")
                .about("Print the character counts")
                .short('m')
                .long("chars"),
        )
        .arg(
            Arg::new("LINES")
                .about("Print the line counts")
                .short('l')
                .long("lines"),
        )
        .arg(
            Arg::new("MAX")
                .about("Print the maximum display width")
                .short('L')
                .long("max-line-length"),
        )
        .arg(
            Arg::new("WORDS")
                .about("Print the word counts")
                .short('w')
                .long("words"),
        )
        .get_matches();
    let flags = get_flags(&matches);
    let files: Vec<_> = match matches.values_of("INPUT") {
        Some(c) => c.collect(),
        None => vec!["-"],
    };
    let mut totals = Values {
        name: "Total".to_string(),
        lines: 0,
        words: 0,
        chars: 0,
        bytes: 0,
        max: 0,
    };
    for file in &files {
        get_values(&file, &mut totals, &flags);
    }
    if files.len() > 1 {
        totals.print_values(&flags);
    }
}
