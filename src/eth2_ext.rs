use crate::Enr;
use ssz::Decode;
use ssz_derive::{Decode, Encode};

/// The ENR field specifying the fork id.
pub const ETH2_ENR_KEY: &str = "eth2";
/// The ENR field specifying the subnet bitfield.
pub const BITFIELD_ENR_KEY: &str = "attnets";

#[derive(Debug, Clone, PartialEq, Default, Encode, Decode, Eq)]
pub struct EnrForkId {
    pub fork_digest: [u8; 4],
    pub next_fork_version: [u8; 4],
    pub next_fork_epoch: u64,
}

/// Extension trait for ENR's within Eth2.
pub trait Eth2Enr {
    /// The subnet bitfield associated with the ENR.
    fn bitfield(&self) -> Option<Vec<u8>>;
    fn eth2(&self) -> Result<EnrForkId, &'static str>;
}

impl Eth2Enr for Enr {
    fn bitfield(&self) -> Option<Vec<u8>> {
        self.get(BITFIELD_ENR_KEY).map(|v| v.to_vec())
    }

    fn eth2(&self) -> Result<EnrForkId, &'static str> {
        let eth2_bytes = self.get(ETH2_ENR_KEY).ok_or("ENR has no eth2 field")?;

        EnrForkId::from_ssz_bytes(eth2_bytes).map_err(|_| "Could not decode EnrForkId")
    }
}
