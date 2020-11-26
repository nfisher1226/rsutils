use clap::{crate_version, App, Arg};

fn main() {
    let matches = App::new("nproc")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Print the number of processing units available")
        .arg(
            Arg::new("ALL")
                .about("Print the number of installed processors")
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
