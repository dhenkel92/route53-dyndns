mod logger;

use clap::{App, Arg};
use std::error::Error;
use logger::initialize_logger;

#[macro_use]
extern crate log;

fn generate_clap_config<'a, 'b>() -> App<'a, 'b> {
    App::new("Route53 DynDns")
        .version("1.0")
        .author("Daniel H. <daniel@henkel.tech>")
        .about("")
        .arg(Arg::with_name("log-file")
            .short("log")
            .long("log-file")
            .help("Path to log file")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbosity")
            .help("Log level verbosity")
            .required(false)
            .possible_values(&["debug", "info", "warn", "error"])
            .default_value("info")
            .takes_value(true))
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = generate_clap_config().get_matches();
    initialize_logger(&matches)?;

    info!("Hello, world!");
    Ok(())
}
