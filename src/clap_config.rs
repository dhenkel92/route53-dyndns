use clap::{App, Arg};

pub fn generate_clap_config<'a, 'b>() -> App<'a, 'b> {
    App::new("Route53 DynDns")
        .version("1.0")
        .author("Daniel H. <daniel@henkel.tech>")
        .about("")
        .arg(
            Arg::with_name("log-file")
                .short("log")
                .long("log-file")
                .help("Path to log file")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbosity")
                .help("Log level verbosity")
                .required(false)
                .possible_values(&["debug", "info", "warn", "error"])
                .default_value("info")
                .takes_value(true),
        )
}
