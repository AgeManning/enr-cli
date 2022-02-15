//! Builds an ENR from CLI params
// use super::Enr;
use crate::eth2_ext::EnrForkId;
use enr::CombinedKey;
use ssz::Decode;
use std::fs::File;
use std::io::prelude::*;

pub fn build(matches: &clap::ArgMatches) -> Result<(), &'static str> {
    // Generate or import a key for the ENR
    let key = {
        let key_bytes = if let Some(priv_key) = matches.value_of("private-key") {
            Some(hex::decode(priv_key).map_err(|_| "Invalid private key hex bytes")?)
        } else if let Some(key_file) = matches.value_of("key-file") {
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

    let mut enr_builder = enr::EnrBuilder::new("v4");

    if let Some(seq) = matches.value_of("seq") {
        enr_builder.seq(seq.parse::<u64>().map_err(|_| "Invalid sequence number")?);
    }
    if let Some(ip) = matches.value_of("ip") {
        enr_builder.ip(ip
            .parse::<std::net::IpAddr>()
            .map_err(|_| "Invalid ip address")?);
    }

    if let Some(tcp) = matches.value_of("tcp-port") {
        enr_builder.tcp(tcp.parse::<u16>().map_err(|_| "Invalid tcp port")?);
    }

    if let Some(udp) = matches.value_of("udp-port") {
        enr_builder.udp(udp.parse::<u16>().map_err(|_| "Invalid udp port")?);
    }

    if let Some(eth2) = matches.value_of("eth2") {
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
