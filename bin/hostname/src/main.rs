#![warn(clippy::all, clippy::pedantic)]
use clap::{Arg, Command};
use std::{io, process};

fn main() -> io::Result<()> {
    let matches = Command::new("hostname")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Prints the name of the current host. The super-user can set the host name by supplying an argument.")
        .arg(
            Arg::new("NAME")
                .help("name to set")
        )
        .arg(
            Arg::new("STRIP")
                .help("Removes any domain information from the printed name.")
                .short('s')
                .long("strip"),
        )
        .get_matches();
    if matches.is_present("NAME") {
        hostname::set(match matches.value_of("NAME") {
            Some(c) => c.to_string(),
            None => {
                eprintln!("hostname: missing operand");
                process::exit(1);
            }
        })?;
    } else {
        let hostname = hostname::get()?;
        if matches.is_present("STRIP") {
            println!(
                "{}",
                match hostname.to_string_lossy().split('.').next() {
                    Some(c) => c,
                    None => {
                        eprintln!("hostname: missing operand");
                        process::exit(1);
                    }
                }
            );
        } else {
            println!("{}", hostname.to_string_lossy());
        }
    }
    Ok(())
}
