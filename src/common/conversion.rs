//! Conversion functions for converting between different types.
use libp2p_core::{identity::PublicKey, PeerId};
use tiny_keccak::{Hasher, Keccak};

/// Converts a peer_id to a node_id.
/// This is only possible for secp256k1/ed25519 libp2p peer_ids.
pub fn peer_id_to_node_id(peer_id: &PeerId) -> Result<enr::NodeId, String> {
    // A libp2p peer id byte representation should be 2 length bytes + 4 protobuf bytes + compressed pk bytes
    // if generated from a PublicKey with Identity multihash.
    let pk_bytes = &peer_id.to_bytes()[2..];

    match PublicKey::from_protobuf_encoding(pk_bytes).map_err(|e| {
        format!(
            " Cannot parse libp2p public key public key from peer id: {}",
            e
        )
    })? {
        PublicKey::Secp256k1(pk) => {
            let uncompressed_key_bytes = &pk.encode_uncompressed()[1..];
            let mut output = [0_u8; 32];
            let mut hasher = Keccak::v256();
            hasher.update(uncompressed_key_bytes);
            hasher.finalize(&mut output);
            Ok(enr::NodeId::parse(&output).expect("Must be correct length"))
        }
        PublicKey::Ed25519(pk) => {
            let uncompressed_key_bytes = pk.encode();
            let mut output = [0_u8; 32];
            let mut hasher = Keccak::v256();
            hasher.update(&uncompressed_key_bytes);
            hasher.finalize(&mut output);
            Ok(enr::NodeId::parse(&output).expect("Must be correct length"))
        }
        _ => Err("Unsupported public key".into()),
    }
}