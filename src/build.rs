//! Builds an ENR from CLI params
// use super::Enr;
use crate::enr_ext::{QUIC6_ENR_KEY, QUIC_ENR_KEY};
use crate::eth2_ext::EnrForkId;
use enr::CombinedKey;
use ssz::Decode;
use std::fs::File;
use std::io::prelude::*;

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
    if let Some(ips) = matches.get_many::<String>("ip") {
        for ip in ips {
            enr_builder.ip(ip
                .parse::<std::net::IpAddr>()
                .map_err(|_| "Invalid ip address")?);
        }
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

#[cfg(test)]
mod tests {
    use crate::enr_ext::{EnrExt, QUIC6_ENR_KEY, QUIC_ENR_KEY};
    use enr::CombinedKey;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_enr_with_quic_ports() {
        // Generate a key
        let key = CombinedKey::generate_secp256k1();

        // Define test values
        let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
        let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let quic4_port: u16 = 9000;
        let quic6_port: u16 = 9001;

        // Create the ENR Builder
        let mut enr_builder = enr::Builder::default();

        // Add IPv4, IPv6, and both QUIC ports
        enr_builder.ip(ipv4.into());
        enr_builder.ip6(ipv6);
        enr_builder.add_value(QUIC_ENR_KEY, &quic4_port.to_be_bytes());
        enr_builder.add_value(QUIC6_ENR_KEY, &quic6_port.to_be_bytes());

        // Build the ENR
        let enr = enr_builder
            .build(&key)
            .expect("Should build ENR successfully");

        // Verify the QUIC ports were correctly added
        let retrieved_quic4_port = enr.quic4();
        let retrieved_quic6_port = enr.quic6();

        assert_eq!(
            retrieved_quic4_port,
            Some(quic4_port),
            "QUIC4 port should be correctly stored and retrieved"
        );
        assert_eq!(
            retrieved_quic6_port,
            Some(quic6_port),
            "QUIC6 port should be correctly stored and retrieved"
        );

        // Verify the multiaddrs include QUIC for both IPv4 and IPv6
        let quic_multiaddrs = enr.multiaddr_quic();
        assert_eq!(
            quic_multiaddrs.len(),
            2,
            "ENR should have both IPv4 and IPv6 QUIC multiaddrs"
        );

        // Verify the correct QUIC multiaddr formats
        let expected_quic4_multiaddr = format!("/ip4/127.0.0.1/udp/{}/quic-v1", quic4_port);
        let expected_quic6_multiaddr = format!("/ip6/::1/udp/{}/quic-v1", quic6_port);

        // Find and check each multiaddr (order is not guaranteed)
        let has_quic4 = quic_multiaddrs
            .iter()
            .any(|addr| addr.to_string() == expected_quic4_multiaddr);
        let has_quic6 = quic_multiaddrs
            .iter()
            .any(|addr| addr.to_string() == expected_quic6_multiaddr);

        assert!(has_quic4, "ENR should have correct IPv4 QUIC multiaddr");
        assert!(has_quic6, "ENR should have correct IPv6 QUIC multiaddr");
    }
}
