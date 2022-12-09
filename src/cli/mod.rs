use clap::{Parser as ClapParser, Subcommand as ClapSubcommand};

use crate::common::logging;

/// enr-cli args
#[allow(missing_docs)]
#[derive(ClapParser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short = 'v',
        long,
        default_value = "info",
        help = "Sets the logging verbosity level."
    )]
    pub log_level: logging::LogLevel,

    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}

/// enr-cli subcommands
#[derive(ClapSubcommand, Clone, Debug)]
#[allow(missing_docs)]
pub enum Subcommand {
    #[clap(name = "build", about = "Builds an ENR")]
    Build(crate::services::build::Build),
    #[clap(name = "read", about = "Reads an ENR")]
    Read(crate::services::read::Read),
}
