use crate::types::*;
use enr_fork_id::EnrForkId;

/// Extension trait for ENR's within Eth2.
pub trait Eth2Enr {
    /// The subnet bitfield associated with the ENR.
    fn bitfield(&self) -> Option<Vec<u8>>;

    /// The Eth2 fork id associated with the ENR.
    fn eth2(&self) -> Result<EnrForkId, &'static str>;
}

