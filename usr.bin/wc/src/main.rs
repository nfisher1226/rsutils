#![warn(clippy::all, clippy::pedantic)]
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use std::io::{stdin, Read};
use std::fmt::Write;
use std::fs;

#[derive(Default)]
struct Values {
    name: String,
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    max: usize,
}

#[derive(Clone)]
enum Flags {
    Lines,
    Words,
    Chars,
    Bytes,
    Max,
}

impl Values {
    fn print_values(&self, flags: &[Flags]) -> Result<(), Box<dyn Error>> {
        let mut line = String::new();
        flags.iter().try_for_each(|flag| {
            match flag {
                Flags::Lines => write!(line,"\t{}", self.lines),
                Flags::Words => write!(line, "\t{}", self.words),
                Flags::Chars => write!(line, "\t{}", self.chars),
                Flags::Bytes => write!(line, "\t{}", self.bytes),
                Flags::Max => write!(line, "\t{}", self.max),
            }
        })?;
        if self.name != "-" {
            write!(line, "\t{}", self.name).unwrap();
        }
        println!("{}", line);
        Ok(())
    }
}

fn get_flags(args: &ArgMatches) -> Vec<Flags> {
    let mut flags: Vec<Flags> = vec![];
    ["LINES", "WORDS", "CHARS", "BYTES", "MAX"].iter().for_each(|arg| {
        if args.is_present(arg) {
            flags.push(match *arg {
                "LINES" => Flags::Lines,
                "WORDS" => Flags::Words,
                "CHARS" => Flags::Chars,
                "BYTES" => Flags::Bytes,
                "MAX" => Flags::Max,
                _ => unreachable!()
            });
        }
    });
    if flags.is_empty() {
        flags.extend_from_slice(&[Flags::Bytes, Flags::Max, Flags::Words]);
    }
    flags
}

fn get_values(file: &str, totals: &mut Values, flags: &[Flags]) -> Result<(), Box<dyn Error>> {
    let contents = if file == "-" {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf
    } else {
        fs::read_to_string(file)?
    };
    let mut f = Values {
        name: (*file).to_string(),
        ..Values::default()
    };
    for flag in flags {
        match flag {
            Flags::Lines => {
                f.lines = contents.lines().count();
                totals.lines += f.lines;
            }
            Flags::Words => {
                f.words = contents.split_whitespace().count();
                totals.words += f.words;
            }
            Flags::Chars => {
                f.chars = contents.chars().count();
                totals.chars += f.words;
            }
            Flags::Bytes => {
                f.bytes = contents.bytes().count();
                totals.bytes += f.bytes;
            }
            Flags::Max => {
                f.max = 0;
                contents.lines().into_iter().for_each(|line| {
                    let max = line.chars().count();
                    if max > f.max {
                        f.max = max;
                    }
                });
                totals.max += f.max;
            }
        };
    }
    Ok(f.print_values(&flags)?)
}

fn main() {
    let matches = Command::new("wc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Print newline, word, and byte counts for each file")
        .arg(
            Arg::new("INPUT")
                .help("The input file to use")
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("BYTES")
                .help("Print the byte counts")
                .short('c')
                .long("bytes"),
        )
        .arg(
            Arg::new("CHARS")
                .help("Print the character counts")
                .short('m')
                .long("chars"),
        )
        .arg(
            Arg::new("LINES")
                .help("Print the line counts")
                .short('l')
                .long("lines"),
        )
        .arg(
            Arg::new("MAX")
                .help("Print the maximum display width")
                .short('L')
                .long("max-line-length"),
        )
        .arg(
            Arg::new("WORDS")
                .help("Print the word counts")
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
        ..Values::default()
    };
    for file in &files {
        if let Err(e) = get_values(&file, &mut totals, &flags) {
            panic!("Error: {}", e);
        }
    }
    if files.len() > 1 {
        if let Err(e) = totals.print_values(&flags) {
            panic!("Error: {}", e);
        }
    }
}
