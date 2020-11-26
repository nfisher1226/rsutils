use clap::{crate_version, App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use textwrap::fill;

fn wrapit_s(line: &str, width: usize) {
    let line = line.replace('\t', "    ");
    println!("{}", fill(line.trim_end(), width));
}

fn wrapit_b(line: &str, width: usize) {
    for (index, b) in line.as_bytes().iter().enumerate() {
        if index % width == 0 {
            println!();
        }
        print!("{}", *b as char);
    }
}

fn wrapit(line: &str, width: usize) {
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

fn wrap_stdin(width: usize, words: bool, bytes: bool) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if words {
            wrapit_s(&line.unwrap(), width);
        } else if bytes {
            wrapit_b(&line.unwrap(), width);
        } else {
            wrapit(&line.unwrap(), width);
        }
    }
}

fn wrap_file(file: String, width: usize, words: bool, bytes: bool) {
    let buf = File::open(file);
    let buf = match buf {
        Ok(buf) => buf,
        Err(m) => panic!("Error opening file: {:?}", m),
    };
    let buf = BufReader::new(buf);
    for line in buf.lines() {
        if words {
            wrapit_s(&line.unwrap(), width);
        } else if bytes {
            wrapit_b(&line.unwrap(), width);
        } else {
            wrapit(&line.unwrap(), width);
        }
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("fold")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Wrap each input line to fit in specified width")
        .arg(
            Arg::new("INPUT")
                .about("The input file to use")
                .multiple(true),
        )
        .arg(
            Arg::new("BYTES")
                .about("Count bytes rather than columns")
                .short('b')
                .long("bytes"),
        )
        .arg(
            Arg::new("WORDS")
                .about("Break at spaces")
                .short('s')
                .long("spaces"),
        )
        .arg(
            Arg::new("WIDTH")
                .about("Use width columns")
                .short('w')
                .long("width")
                .default_value("80")
                .takes_value(true),
        )
        .get_matches();
    let width: usize = matches.value_of_t("WIDTH").unwrap();
    let mut bytes: bool = false;
    if matches.is_present("BYTES") {
        bytes = true;
    }
    let mut words: bool = false;
    if matches.is_present("WORDS") {
        words = true;
    }
    if matches.is_present("INPUT") {
        let files: Vec<_> = matches.values_of("INPUT").unwrap().collect();
        for file in files {
            if file == "-" {
                wrap_stdin(width, words, bytes);
            } else {
                wrap_file(file.to_string(), width, words, bytes);
            }
            if bytes {
                println!();
            }
        }
    } else {
        wrap_stdin(width, words, bytes);
    }
    Ok(())
}
