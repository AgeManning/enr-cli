use clap::Args;

/// Build Command Options
///
/// Builds a base64 ENR from a private key and parameters.
#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Build {
    /// A hex encoded private key to use for signing. If this or --key-file is not specified a random one will be generated.
    #[clap(
        short = 'k',
        long = "private-key",
        allow_hyphen_values = true,
        help = "A hex encoded private key to use for signing. If this or --key-file is not specified a random one will be generated."
    )]
    pub private_key: Option<String>,
    /// Path to a key file that stores raw bytes of an ENR key.
    /// Example for lighthouse is in ~/.lighthouse/mainnet/beacon/network/key.dat.
    /// If this or --private-key is not specified a random one will be generated.
    #[clap(
        short = 'j',
        long = "key-file",
        allow_hyphen_values = true,
        help = "Path to a key file that stores raw bytes of an ENR key. Example for lighthouse is in ~/.lighthouse/mainnet/beacon/network/key.dat."
    )]
    pub key_file: Option<String>,
    /// Set an ip address
    #[clap(
        long = "ip",
        short = 'i',
        help = "Set an ip address"
    )]
    pub ip: Option<String>,
    /// Set a sequence number
    #[clap(
        long = "seq-no",
        short = 's',
        help = "Set a sequence number"
    )]
    pub seq: Option<String>,
    /// Set an tcp port
    #[clap(
        long = "tcp-port",
        short = 'p',
        help = "Set an tcp port"
    )]
    pub tcp_port: Option<u16>,
    /// Set an udp port
    #[clap(
        long = "udp-port",
        short = 'u',
        help = "Set an udp port"
    )]
    pub udp_port: Option<u16>,
    /// Set an eth2 fork field. Takes the raw SSZ bytes input
    #[clap(
        long = "eth2",
        short = 'f',
        help = "Set an eth2 fork field. Takes the raw SSZ bytes input"
    )]
    pub eth2: Option<String>,
}

