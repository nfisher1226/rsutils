use clap::{crate_version, App, Arg};
use std::fs;
use std::io::{stdin, Read};

fn head(file: &str, count: usize, header: bool, bytes: bool) {
    let mut contents = String::new();
    if file == "-" {
        stdin().read_to_string(&mut contents).unwrap();
    } else {
        contents = fs::read_to_string(file).unwrap();
    }
    if header {
        println!("==> {} <==", file);
    }
    let mut index = 0;
    if bytes {
        for char in contents.chars() {
            if index < count {
                print!("{}", char);
            }
            index += 1;
            if index == count {
                println!();
                return;
            }
        }
        println!();
    } else {
        for line in contents.lines() {
            if index < count {
                println!("{}", line);
            }
            index += 1;
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
    		    .about("Each file is preceded by a header consisting of the string “==> XXX ≤==” where “XXX” is the name of the file.")
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
    let mut bytes: bool = false;
    if matches.is_present("BYTES") {
        bytes = true;
    }
    let mut header: bool = false;
    if matches.is_present("HEADER") && !matches.is_present("QUIET") {
        header = true;
    }
    if matches.is_present("FILES") {
        let files: Vec<_> = matches.values_of("FILES").unwrap().collect();
        if !matches.is_present("QUIET") && files.len() > 1 {
            header = true;
        }
        let mut i = 0;
        for file in files {
            if i == 1 && header {
                println!();
            }
            head(file, count, header, bytes);
            i = 1;
        }
    } else {
          head("-", count, header, bytes);
    }
}
