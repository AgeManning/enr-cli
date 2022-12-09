use clap::Args;

/// Read Command Options
///
/// Reads a base64 ENR and prints common parameters.
#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Read {
    /// The base64-encoded ENR.
    #[clap(
        index = 1,
        required = true,
        help = "The base64-encoded ENR."
    )]
    pub enr: String,
}