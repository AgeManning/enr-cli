#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

//! Core library for enr-cli.

/// Cli
pub mod cli;

/// Extension Traits for enr-cli.
pub mod extensions;

/// Common utilities for enr-cli.
pub mod common;

/// Core Services for enr-cli.
pub mod services;

/// Types for enr-cli.
pub mod types;

/// Re-export core components across public modules.
pub mod prelude {
    pub use super::cli::*;
    pub use super::services::*;
    pub use super::common::prelude::*;
    pub use super::extensions::*;
    pub use super::types::*;
}

