#![warn(clippy::all, clippy::pedantic)]
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("nproc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Print the number of processing units available")
        .arg(
            Arg::new("ALL")
                .help("Print the number of installed processors")
                .short('a')
                .long("all"),
        )
        .get_matches();
    if matches.is_present("ALL") {
        println!("{}", num_cpus::get());
    } else {
        println!("{}", num_cpus::get_physical());
    }
}
