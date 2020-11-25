use clap::{crate_version, App, Arg};
use std::{io, process};

fn main() -> io::Result<()> {
    let matches = App::new("hostname")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Prints the name of the current host. The super-user can set the host name by supplying an argument.")
        .arg(
            Arg::new("NAME")
                .about("name to set")
                .multiple(true),
        )
        .arg(
            Arg::new("STRIP")
                .about("Removes any domain information from the printed name.")
                .short('s')
                .long("strip"),
        )
        .get_matches();
    let mut strip = false;
    if matches.is_present("STRIP") {
        strip = true;
    }
    if !matches.is_present("NAME") {
        let hostname = hostname::get()?;
        if strip {
            println!("{}", hostname.to_string_lossy().split(".").next().unwrap());
        } else {
            println!("{}", hostname.to_string_lossy());
        }
    } else {
        let hostname = matches.value_of("NAME").unwrap().to_string();
        let result = hostname::set(hostname);
        match result {
      			Ok(c) => c,
			      Err(m) => {
				        eprintln!("Error: sethostname: {}", m);
				        process::exit(1);
			      }
		    };
    }
    Ok(())
}
