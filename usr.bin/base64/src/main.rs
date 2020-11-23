use base64::{decode, encode};
use clap::{crate_version, App, Arg};
use std::fs;
use std::io::{self, Read};

fn main() {
    let matches = App::new("base32")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("base64 encode/decode data and print to standard output")
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
        .get_matches();

    let mut contents = String::new();
    if matches.is_present("INPUT") {
        let file = matches.value_of("INPUT").unwrap().to_string();
        contents = fs::read_to_string(&file).unwrap();
    } else {
        io::stdin().read_to_string(&mut contents).unwrap();
    }
    if matches.is_present("DECODE") {
        contents = contents.replace('\n', "");
        let decoded = &decode(contents).unwrap();
        let output = String::from_utf8(decoded.to_vec()).unwrap();
        println!("{}", output.trim_end());
    } else {
        let encoded = encode(&contents.as_bytes())
            .chars()
            .collect::<Vec<char>>()
            .chunks(76)
            .map(|c| c.iter()
            .collect::<String>())
            .collect::<Vec<String>>();
        for line in encoded.iter() {
            println!("{}", line);
        }
    }
}
