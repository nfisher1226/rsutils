#![warn(clippy::all, clippy::pedantic)]
use clap::{Arg, Command};
use std::fs;
use std::io::{stdin, Read};
use std::process;

fn head(file: &str, count: usize, header: bool, bytes: bool) {
    let mut contents = String::new();
    if file == "-" {
        match stdin().read_to_string(&mut contents) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("head: {}", e);
                process::exit(1);
            }
        };
    } else {
        let buf = fs::read_to_string(file);
        contents = match buf {
            Ok(c) => c,
            Err(e) => {
                eprintln!("head: {}", e);
                process::exit(1);
            }
        };
    }
    if header {
        println!("==> {} <==", file);
    }
    if bytes {
        for (index, char) in contents.chars().into_iter().enumerate() {
            if index < count {
                print!("{}", char);
            } else {
                println!();
                return;
            }
        }
        println!();
    } else {
        for (index, line) in contents.lines().into_iter().enumerate() {
            if index < count {
                println!("{}", line);
            } else {
                return;
            }
        }
    }
}

fn main() {
    let matches = Command::new("head")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Display first lines of a file")
        .arg(
            Arg::new("FILES")
                .help("The input file to use")
                .multiple_occurrences(true),
        )
        .arg(
            Arg::new("BYTES")
                .help("Count bytes instead of lines")
                .short('c')
                .long("bytes"),
        )
    		.arg(
      			Arg::new("QUIET")
     				.help("Disable printing a header. Overrides -c")
    				.short('q')
    				.long("quiet")
    		)
    		.arg(
    		    Arg::new("HEADER")
    		    .help("Each file is preceded by a header consisting of the string \"==> XXX <==\" where \"XXX\" is the name of the file.")
    		    .short('v')
    		    .long("verbose")
		    )
        .arg(
      			Arg::new("LINES")
    				.help("Count n number of lines (or bytes if -c is specified).")
    				.short('n')
    				.long("lines")
    				.default_value("10")
    				.takes_value(true),
    		)
        .get_matches();

    let files: Vec<_> = match matches.values_of("FILES") {
        Some(c) => c.collect(),
        None => vec!["-"],
    };
    let header: bool =
        !matches.is_present("QUIET") && { files.len() > 1 || matches.is_present("HEADER") };
    for (index, file) in files.into_iter().enumerate() {
        if index == 1 && header {
            println!();
        }
        head(
            file,
            match matches.value_of_t("LINES") {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("head: {}", e);
                    process::exit(1);
                }
            },
            header,
            matches.is_present("BYTES"),
        );
    }
}
