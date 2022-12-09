#![doc = include_str!("../README.md")]

use clap::Parser;
use enr_cli::cli;

fn main() {
    // Parse the command line arguments
    let cli = enr_cli::cli::Cli::parse();

    // Setup logging using the specified log level
    enr_cli::common::logging::construct_simple_logger(cli.log_level);

    // Run the appropriate command
    match cli.subcommand {
        Some(cli::Subcommand::Read(ref e)) => {
            if let Err(e) = enr_cli::services::read::run(e) {
                log::error!("Failed to read ENR: {}", e);
            }
        }
        Some(cli::Subcommand::Build(ref b)) => {
            if let Err(e) = enr_cli::services::build::run(b) {
                log::error!("Failed to build ENR: {}", e);
            }
        }
        _ => log::error!("Unable to parse command line arguments. See --help for options"),
    }
}