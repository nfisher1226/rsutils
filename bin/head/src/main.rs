#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg};
use std::fs;
use std::io::{stdin, Read};

fn head(file: &str, count: usize, header: bool, bytes: bool) {
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
    if header {
        println!("==> {} <==", file);
    }
    if bytes {
        for (index, char) in contents.chars().into_iter().enumerate() {
            if index < count {
                print!("{}", char);
            }
            if index == count {
                println!();
                return;
            }
        }
        println!();
    } else {
        for (index, line) in contents.lines().into_iter().enumerate() {
            if index < count {
                println!("{}", line);
            }
            if index == count {
                return;
            }
        }
    }
}

fn main() {
    let matches = App::new("head")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Display first lines of a file")
        .arg(
            Arg::new("FILES")
                .about("The input file to use")
                .multiple(true),
        )
        .arg(
            Arg::new("BYTES")
                .about("Count bytes instead of lines")
                .short('c')
                .long("bytes"),
        )
    		.arg(
      			Arg::new("QUIET")
     				.about("Disable printing a header. Overrides -c")
    				.short('q')
    				.long("quiet")
    		)
    		.arg(
    		    Arg::new("HEADER")
    		    .about("Each file is preceded by a header consisting of the string \"==> XXX <==\" where \"XXX\" is the name of the file.")
    		    .short('v')
    		    .long("verbose")
		    )
        .arg(
      			Arg::new("LINES")
    				.about("Count n number of lines (or bytes if -c is specified).")
    				.short('n')
    				.long("lines")
    				.default_value("10")
    				.takes_value(true),
    		)
        .get_matches();

    let count: usize = matches.value_of_t("LINES").unwrap();
    let bytes: bool = matches.is_present("BYTES");
    let files: Vec<_> = if matches.is_present("FILES") {
        matches.values_of("FILES").unwrap().collect()
    } else {
        vec!["-"]
    };
    let header: bool = !matches.is_present("QUIET") && { files.len() > 1 || matches.is_present("HEADER") };
    for (index, file) in files.into_iter().enumerate() {
        if index == 1 && header {
            println!();
        }
        head(file, count, header, bytes);
    }
}
