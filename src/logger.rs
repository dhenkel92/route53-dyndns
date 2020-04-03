use clap::ArgMatches;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, SimpleLogger, WriteLogger};
use std::fs::File;
use std::{error::Error, path::Path};

fn verbosity_to_level_filter(verbosity: &str) -> LevelFilter {
    match verbosity {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}

pub fn initialize_logger(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let verbosity = args.value_of("verbosity").unwrap_or("info");
    let level_filter = verbosity_to_level_filter(verbosity);

    if let Some(log_file) = args.value_of("log-file") {
        let log_file_path = Path::new(log_file);
        CombinedLogger::init(vec![
            SimpleLogger::new(level_filter, Config::default()),
            WriteLogger::new(
                level_filter,
                Config::default(),
                File::create(log_file_path)?,
            ),
        ])?;
    } else {
        CombinedLogger::init(vec![SimpleLogger::new(level_filter, Config::default())])?;
    }

    Ok(())
}
