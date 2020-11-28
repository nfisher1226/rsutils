#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg};
use std::fs;
use std::io::{stdin, Read};

struct Values {
    name: String,
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    max: usize,
}

fn print_values(values: &Values, flags: &[char]) {
    let mut line = String::new();
    for flag in flags {
        match flag {
            'l' => line = format!("{}\t{}", line, values.lines),
            'w' => line = format!("{}\t{}", line, values.words),
            'm' => line = format!("{}\t{}", line, values.chars),
            'c' => line = format!("{}\t{}", line, values.bytes),
            'L' => line = format!("{}\t{}", line, values.max),
            _ => panic!("Illegal input"),
        }
    }
    if values.name != "-" {
        line = format!("{}\t{}", line, values.name);
    }
    println!("{}", line);
}

fn get_values(file: &str, flags: &[char]) -> Values {
    let mut contents = String::new();
    if file == "-" {
        stdin().read_to_string(&mut contents).unwrap();
    } else {
        let buf = fs::read_to_string(file);
        contents = match buf {
            Ok(c) => c,
            Err(m) => panic!("Error opening file: {:?}", m),
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
            'l' => f.lines = contents.lines().count(),
            'w' => f.words = contents.split_whitespace().count(),
            'm' => f.chars = contents.chars().count(),
            'c' => f.bytes = contents.bytes().count(),
            'L' => {
                f.max = 0;
                for line in contents.lines() {
                    let max = line.chars().count();
                    if max > f.max {
                        f.max = max;
                    }
                }
            }
            _ => panic!("Illegal input"),
        };
    }
    print_values(&f, &flags);
    f
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
    let mut flags = Vec::new();
    if matches.is_present("LINES") {
        flags.push('l');
    }
    if matches.is_present("WORDS") {
        flags.push('w');
    }
    if matches.is_present("CHARS") {
        flags.push('m');
    }
    if matches.is_present("BYTES") {
        flags.push('c');
    }
    if matches.is_present("MAX") {
        flags.push('L');
    }
    if flags.is_empty() {
        flags.push('c');
        flags.push('l');
        flags.push('w');
    }
    let files: Vec<_> = if matches.is_present("INPUT") {
        matches.values_of("INPUT").unwrap().collect()
    } else {
        vec!["-"]
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
        let v = get_values(&file, &flags);
        totals.lines += v.lines;
        totals.words += v.words;
        totals.chars += v.words;
        totals.bytes += v.bytes;
        totals.max += v.max;
    }
    if files.len() > 1 {
        print_values(&totals, &flags);
    }
}
