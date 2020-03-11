//! # enr-cli
//!
//! This is currently a simple tool to read base64 encoded ENRs. More features may be added in
//! the future.
//!
//! ## Install
//!
//! This can be installed via cargo:
//!
//! ```bash
//! $ cargo install enr-cli
//! ```
//!
//! ## Usage
//!
//! ```bash
//! Sigma Prime <contact@sigmaprime.io>
//! Simple CLI for reading and modifying ENRs.
//!
//! USAGE:
//!     enr-cli --enr <BASE64-ENR>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -e, --enr <BASE64-ENR>    Reads a base64 ENR and prints common parameters.
//! ```
//!
//! ## Example
//!
//! ```bash
//! $ enr-cli -e -Iu4QM-YJF2RRpMcZkFiWzMf2kRd1A5F1GIekPa4Sfi_v0DCLTDBfOMTMMWJhhawr1YLUPb5008CpnBKrgjY3sstjfgCgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQP8u1uyQFyJYuQUTyA1raXKhSw1HhhxNUQ2VE52LNHWMIN0Y3CCIyiDdWRwgiMo
//! ENR Read
//! Sequence No: 2
//! Node ID: 0x2ba1..acde
//! IP: 127.0.0.1
//! TCP port: 9000
//! UDP: 9000
//! ```

use clap::{App, Arg};
use enr::{CombinedKey, Enr, EnrKey};

fn main() {
    // Parse the CLI parameters.
    let matches = App::new("enr-cli")
        .version("0.1.0")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Simple CLI for reading and modifying ENRs.")
        .arg(
            Arg::with_name("enr")
                .short("e")
                .long("enr")
                .value_name("BASE64-ENR")
                .allow_hyphen_values(true)
                .required(true)
                .help("Reads a base64 ENR and prints common parameters.")
                .takes_value(true),
        )
        .get_matches();

    let enr_base64 = matches.value_of("enr").expect("Must supply an ENR");
    let enr = enr_base64.parse::<Enr<CombinedKey>>().unwrap();
    print_enr(enr);
}

fn print_enr<K: EnrKey>(enr: Enr<K>) {
    println!("ENR Read");
    println!("Sequence No: {}", enr.seq());
    println!("Node ID: {}", enr.node_id());

    if let Some(ip) = enr.ip() {
        println!("IP: {:?}", ip);
    }
    if let Some(tcp) = enr.tcp() {
        println!("TCP Port: {:?}", tcp);
    }
    if let Some(udp) = enr.udp() {
        println!("UDP Port: {:?}", udp);
    }
}
