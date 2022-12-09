//! Types used across the enr-cli crate.

/// EnrForkId Type
pub mod enr_fork_id;

/// ENR Type
pub mod enr;

/// Re-export core components across public modules.
pub mod prelude {
    pub use super::enr::*;
    pub use super::enr_fork_id::*;
}