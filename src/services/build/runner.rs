//! Builds an ENR from CLI params
// use super::Enr;
use ssz::Decode;
use crate::common::key;
use crate::types::enr_fork_id::EnrForkId;

/// Runs the build service.
pub fn run(r: &super::Build) -> eyre::Result<()> {
    // Parse Private Key from CLI as Bytes
    let mut key_bytes = r.private_key.as_deref().map(|k| hex::decode(k).map_err(|_| eyre::eyre!("Invalid private key hex bytes"))).transpose()?;
    if key_bytes.is_none() {
        key_bytes = r.key_file.as_deref().map(key::read_from_file).transpose()?;
    }

    // Generate [CombinedKey](enr::CombinedKey) from Bytes
    let key = key::generate(key_bytes.as_deref_mut())?;

    // Build the enr
    let mut enr_builder = enr::EnrBuilder::new("v4");

    if let Some(seq) = r.seq.as_deref() {
        enr_builder.seq(seq.parse::<u64>().map_err(|_| eyre::eyre!("Invalid sequence number"))?);
    }
    if let Some(ip) = r.ip.as_deref() {
        enr_builder.ip(ip
            .parse::<std::net::IpAddr>()
            .map_err(|_| eyre::eyre!("Invalid ip address"))?);
    }

    if let Some(tcp) = r.tcp_port {
        enr_builder.tcp4(tcp);
    }

    if let Some(udp) = r.udp_port {
        enr_builder.udp4(udp);
    }

    if let Some(eth2) = r.eth2.as_deref() {
        let eth2_bytes = hex::decode(eth2).map_err(|_| eyre::eyre!("Invalid eth2 hex bytes"))?;
        EnrForkId::from_ssz_bytes(&eth2_bytes).map_err(|_| eyre::eyre!("Invalid eth2 ssz bytes"))?;
        enr_builder.add_value("eth2", &eth2_bytes);
    }

    let enr = match enr_builder.build(&key) {
        Err(e) => {
            eyre::bail!("Failed to build ENR\n{:?}", e);
        }
        Ok(v) => v,
    };

    log::info!("Built ENR: {}", enr.to_base64());
    log::info!("");

    // !! Warning this should be removed in the future or hidden behind a specific cli flag !! //
    log::debug!("Private Key: {}", hex::encode(key.encode()));
    log::debug!("");

    // Print the ENR
    crate::common::output::print_enr(enr);

    Ok(())
}
