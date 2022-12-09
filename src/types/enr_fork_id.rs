use ssz_derive::{Encode, Decode};

/// The ENR field specifying the fork id.
pub const ETH2_ENR_KEY: &str = "eth2";

/// The ENR field specifying the subnet bitfield.
pub const BITFIELD_ENR_KEY: &str = "attnets";

/// ENR Fork ID.
#[derive(Debug, Clone, PartialEq, Default, Encode, Decode, Eq)]
pub struct EnrForkId {
    /// Fork digest
    pub fork_digest: [u8; 4],
    /// Next fork version
    pub next_fork_version: [u8; 4],
    /// Next fork epoch
    pub next_fork_epoch: u64,
}
