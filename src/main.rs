use clap::App;
use clap::load_yaml;
use log::trace;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, WriteLogger};
use std::fs::File;

fn main() {
    // configure the command line parser first (since we need the verbosity level for the logger)
    let cli_configuration_yaml = load_yaml!("cli.yml");
    let argument_matches = App::from_yaml(cli_configuration_yaml).get_matches();

    // determine the correct logging level
    let logging_level = match argument_matches.occurrences_of("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    // configure the logging framework and set the corresponding log level
    CombinedLogger::init(vec![
        TermLogger::new(logging_level, Config::default()).unwrap(),
        WriteLogger::new(
            logging_level,
            Config::default(),
            File::create("motion-flow.log").unwrap(),
        ),
    ])
        .unwrap();

    // just log that the basic application has started now
    trace!("Application started");
}
