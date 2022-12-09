use ssz::Encode;

use crate::{types::enr::Enr, extensions::{prelude::EnrExt, eth2::Eth2Enr}};

/// Prints an ENR with [log].
pub fn print_enr(enr: Enr) {
    log::info!("ENR Read:");
    log::info!("Sequence No:{}", enr.seq());
    log::info!("NodeId: {}", hex::encode(enr.node_id().raw()));
    log::info!("EnodeId: {}", enr.enode_id());
    log::info!("Libp2p PeerId: {}", enr.peer_id());
    if let Some(ip) = enr.ip4() {
        log::info!("IP:{:?}", ip);
    }
    if let Some(ip) = enr.ip6() {
        log::info!("IP6:{:?}", ip);
    }
    if let Some(tcp) = enr.tcp4() {
        log::info!("TCP Port:{}", tcp);
    }
    if let Some(tcp) = enr.tcp6() {
        log::info!("TCP6 Port:{}", tcp);
    }
    if let Some(udp) = enr.udp4() {
        log::info!("UDP Port:{}", udp);
    }
    if let Some(udp) = enr.udp6() {
        log::info!("UDP6 Port:{}", udp);
    }

    if let Ok(enr_fork_id) = enr.eth2() {
        log::info!(
            "Eth2 Field:\n\tFork digest: {}\n\tNext fork version: {}\n\tNext fork epoch: {}\n\tSSZ Bytes: {}",
            hex::encode(enr_fork_id.fork_digest),
            hex::encode(enr_fork_id.next_fork_version),
            enr_fork_id.next_fork_epoch,
            hex::encode(enr_fork_id.as_ssz_bytes())
        );
    }

    let multiaddrs = enr.multiaddr();
    if !multiaddrs.is_empty() {
        log::info!("Known multiaddrs:");
        for multiaddr in multiaddrs {
            log::info!("{}", multiaddr);
        }
    }
}
