#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg};
use data_encoding::BASE32;
use std::fs;
use std::io::{self, Read};
use std::process;

fn decode_base32(mut contents: String, ignore: bool) {
    if ignore {
        contents.retain(|c| !c.is_whitespace());
    } else {
        contents = contents.replace('\n', "");
    }
    let decoded = match BASE32.decode(&contents.as_bytes()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("base32: {}", e);
            process::exit(1);
        }
    };
    let output = match String::from_utf8(decoded) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("base32: {}", e);
            process::exit(1);
        }
    };
    println!("{}", output.trim_end());
}

fn encode_base32(contents: &str, wrap: usize) {
    let encoded = BASE32
        .encode(contents.as_bytes())
        .chars()
        .collect::<Vec<char>>()
        .chunks(wrap)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();
    for line in &encoded {
        println!("{}", line);
    }
}

fn get_contents(file: &str) -> String {
    let mut contents = String::new();
    if file == "-" {
        match io::stdin().read_to_string(&mut contents) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("base32: {}", e);
                process::exit(1);
            }
        };
    } else {
        contents = match fs::read_to_string(&file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("base32: {}", e);
                process::exit(1);
            }
        };
    }
    contents
}

fn main() {
    let matches = App::new("base32")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Base32 encode/decode data and print to standard output")
        .arg(
            Arg::new("INPUT")
                .about("The input file to use")
                .multiple(true),
        )
        .arg(
            Arg::new("DECODE")
                .about("Decode rather than encode")
                .short('d')
                .long("decode"),
        )
        .arg(
            Arg::new("IGNORE")
                .about("Ignore whitespace when decoding")
                .short('i')
                .long("ignore-space"),
        )
        .arg(
            Arg::new("WRAP")
                .about("Wrap encoded lines after n characters")
                .short('w')
                .long("wrap")
                .default_value("76")
                .takes_value(true),
        )
        .arg(
            Arg::new("VERBOSE")
                .about("Display a header naming each file")
                .short('v')
                .long("verbose"),
        )
        .arg(
            Arg::new("QUIET")
                .about("Do not display header, even with multiple files")
                .short('q')
                .long("quiet"),
        )
        .get_matches();
    let files: Vec<_> = match matches.values_of("INPUT") {
        Some(c) => c.collect(),
        None => vec!["-"],
    };
    let len = files.len();
    for (index, file) in files.into_iter().enumerate() {
        if { len > 1 || matches.is_present("VERBOSE") } && !matches.is_present("QUIET") {
            match index {
                0 => println!("===> {} <===", file),
                _ => println!("\n===> {} <===", file),
            };
        } else if index > 0 {
            println!();
        }
        let contents = get_contents(file);
        if matches.is_present("DECODE") {
            decode_base32(contents, matches.is_present("IGNORE"));
        } else {
            encode_base32(
                &contents,
                match matches.value_of_t("WRAP") {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("base32: {}", e);
                        process::exit(1);
                    }
                },
            );
        }
    }
}
