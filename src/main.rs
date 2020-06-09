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
//!     enr-cli <BASE64-ENR>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! ARGS:
//!     <BASE64-ENR>    Reads a base64 ENR and prints common parameters.
//! ```
//!
//! ## Example
//!
//! ```bash
//! $ enr-cli enr:-Ku4QJsxkOibTc9FXfBWYmcdMAGwH4bnOOFb4BlTHfMdx_f0WN-u4IUqZcQVP9iuEyoxipFs7-Qd_rH_0HfyOQitc7IBh2F0dG5ldHOIAAAAAAAAAACEZXRoMpD1pf1CAAAAAP__________gmlkgnY0gmlwhLAJM9iJc2VjcDI1NmsxoQL2RyM26TKZzqnUsyycHQB4jnyg6Wi79rwLXtaZXty06YN1ZHCCW8w
//! ENR Read:
//! Sequence No:1
//! NodeId:0x3ab5..1447
//! Libp2p PeerId:16Uiu2HAmC13Brucnz5qR8caKi8qKK6766PFoxsF5MzK2RvbTyBRr
//! IP:176.9.51.216
//! UDP Port:23500
//! Known multiaddrs:
//! /ip4/176.9.51.216/udp/23500/p2p/16Uiu2HAmC13Brucnz5qR8caKi8qKK6766PFoxsF5MzK2RvbTyBRr
//! ```

use clap::{App, Arg};
use enr::{CombinedKey, Enr as EnrRaw};
mod enr_ext;
use enr_ext::EnrExt;

pub type Enr = EnrRaw<CombinedKey>;

fn main() {
    // Parse the CLI parameters.
    let matches = App::new("enr-cli")
        .version("0.1.0")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Simple CLI for reading and modifying ENRs.")
        .arg(
            Arg::with_name("enr")
                .value_name("BASE64-ENR")
                .allow_hyphen_values(true)
                .required(true)
                .help("Reads a base64 ENR and prints common parameters.")
                .takes_value(true),
        )
        .get_matches();

    let enr_base64 = matches.value_of("enr").expect("Must supply an ENR");
    let enr = enr_base64.parse::<Enr>().unwrap();
    print_enr(enr);
}

fn print_enr(enr: Enr) {
    println!("ENR Read:");
    println!("Sequence No:{}", enr.seq());
    println!("NodeId:{}", enr.node_id());
    println!("Libp2p PeerId:{}", enr.peer_id());
    if let Some(ip) = enr.ip() {
        println!("IP:{:?}", ip);
    }
    if let Some(tcp) = enr.tcp() {
        println!("TCP Port:{}", tcp);
    }
    if let Some(udp) = enr.udp() {
        println!("UDP Port:{}", udp);
    }

    let multiaddrs = enr.multiaddr();
    if !multiaddrs.is_empty() {
        println!("Known multiaddrs:");
        for multiaddr in multiaddrs {
            println!("{}", multiaddr);
        }
    }
}
