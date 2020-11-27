#![warn(clippy::all, clippy::pedantic)]
use base64::{decode, encode};
use clap::{crate_version, App, Arg};
use std::fs;
use std::io::{self, Read};

fn main() {
    let matches = App::new("base64")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("base64 encode/decode data and print to standard output")
        .arg(
            Arg::new("INPUT")
                .about("The input file to use")
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
        .get_matches();
    let wrap: usize = matches.value_of_t("WRAP").unwrap();
    let mut contents = String::new();
    if matches.is_present("INPUT") {
        let file = matches.value_of("INPUT").unwrap().to_string();
        contents = fs::read_to_string(&file).unwrap();
    } else {
        io::stdin().read_to_string(&mut contents).unwrap();
    }
    if matches.is_present("DECODE") {
        if matches.is_present("IGNORE") {
            contents.retain(|c| !c.is_whitespace());
        } else {
            contents = contents.replace('\n', "");
        }
        let decoded = &decode(contents).unwrap();
        let output = String::from_utf8(decoded.to_vec()).unwrap();
        println!("{}", output.trim_end());
    } else {
        let encoded = encode(&contents.as_bytes())
            .chars()
            .collect::<Vec<char>>()
            .chunks(wrap)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        for line in &encoded {
            println!("{}", line);
        }
    }
}
