use clap::load_yaml;
use clap::App;
use log::{error, trace};
use motion_flow::subcommands::flowanalysis::FlowAnalysis;
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
        2 | _ => LevelFilter::Trace,
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

    // based on the correct sub-command, select the module to run it
    let sub_command = match argument_matches.subcommand_name().unwrap() {
        "flowanalysis" => FlowAnalysis::new(
            argument_matches
                .subcommand_matches("flowanalysis")
                .unwrap()
                .value_of("input_folder")
                .unwrap(),
            argument_matches
                .subcommand_matches("flowanalysis")
                .unwrap()
                .value_of("pattern")
                .unwrap(),
        ),
        _ => panic!("Unknown sub-command selected."),
    };

    //
    if sub_command.is_err() {
        error!(
            "Could not create an instance of the sub-command. The error was: {:?}",
            sub_command.err().unwrap()
        )
    }
}
