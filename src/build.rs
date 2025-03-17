//! Builds an ENR from CLI params
// use super::Enr;
use crate::enr_ext::{QUIC6_ENR_KEY, QUIC_ENR_KEY};
use crate::eth2_ext::EnrForkId;
use enr::CombinedKey;
use ssz::Decode;
use std::fs::File;
use std::io::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn build(matches: &clap::ArgMatches) -> Result<(), &'static str> {
    // Generate or import a key for the ENR
    let key = {
        let key_bytes = if let Some(priv_key) = matches.get_one::<String>("private-key") {
            Some(hex::decode(priv_key).map_err(|_| "Invalid private key hex bytes")?)
        } else if let Some(key_file) = matches.get_one::<String>("key-file") {
            let mut file = File::open(key_file).map_err(|_| "Cannot find key-file")?;
            let mut key_bytes: Vec<u8> = Vec::with_capacity(36);
            file.read_to_end(&mut key_bytes)
                .map_err(|_| "Cannot read key file")?;
            Some(key_bytes)
        } else {
            None
        };

        if let Some(mut byte_key) = key_bytes {
            if let Ok(combined_key) = CombinedKey::secp256k1_from_bytes(&mut byte_key) {
                combined_key
            } else if let Ok(combined_key) = CombinedKey::ed25519_from_bytes(&mut byte_key) {
                combined_key
            } else {
                return Err("Invalid private key");
            }
        } else {
            CombinedKey::generate_secp256k1()
        }
    };

    // Build the ENR:
    let mut enr_builder = enr::Builder::default();

    if let Some(seq) = matches.get_one::<String>("seq") {
        enr_builder.seq(seq.parse::<u64>().map_err(|_| "Invalid sequence number")?);
    }
    if let Some(ip4) = matches.get_one::<String>("ip4") {
        let ipv4 = ip4
            .parse::<Ipv4Addr>()
            .map_err(|_| "Invalid IPv4 address")?;
        enr_builder.ip4(ipv4);
    }
    if let Some(ip6) = matches.get_one::<String>("ip6") {
        let ipv6 = ip6
            .parse::<Ipv6Addr>()
            .map_err(|_| "Invalid IPv6 address")?;
        enr_builder.ip6(ipv6);
    }

    if let Some(tcp) = matches.get_one::<String>("tcp-port") {
        enr_builder.tcp4(tcp.parse::<u16>().map_err(|_| "Invalid tcp port")?);
    }
    if let Some(tcp6) = matches.get_one::<String>("tcp6-port") {
        enr_builder.tcp6(tcp6.parse::<u16>().map_err(|_| "Invalid tcp6 port")?);
    }

    if let Some(udp) = matches.get_one::<String>("udp-port") {
        enr_builder.udp4(udp.parse::<u16>().map_err(|_| "Invalid udp port")?);
    }
    if let Some(udp6) = matches.get_one::<String>("udp6-port") {
        enr_builder.udp6(udp6.parse::<u16>().map_err(|_| "Invalid udp6 port")?);
    }

    if let Some(quic) = matches.get_one::<String>("quic-port") {
        enr_builder.add_value(
            QUIC_ENR_KEY,
            &quic
                .parse::<u16>()
                .map_err(|_| "Invalid quic port")?
                .to_be_bytes(),
        );
    }
    if let Some(quic6) = matches.get_one::<String>("quic6-port") {
        enr_builder.add_value(
            QUIC6_ENR_KEY,
            &quic6
                .parse::<u16>()
                .map_err(|_| "Invalid quic6 port")?
                .to_be_bytes(),
        );
    }

    if let Some(eth2) = matches.get_one::<String>("eth2") {
        let eth2_bytes = hex::decode(eth2).map_err(|_| "Invalid eth2 hex bytes")?;
        EnrForkId::from_ssz_bytes(&eth2_bytes).map_err(|_| "Invalid eth2 ssz bytes")?;

        enr_builder.add_value("eth2", &eth2_bytes);
    }

    let enr = match enr_builder.build(&key) {
        Err(e) => {
            println!("Failed to build ENR: {:?}", e);
            return Err("Failed to build ENR");
        }
        Ok(v) => v,
    };

    println!("Built ENR: {}", enr.to_base64());
    println!();
    println!("Private Key: {}", hex::encode(key.encode()));
    println!();

    super::print_enr(enr);

    Ok(())
}
