
use crate::{common, types::prelude::Enr};

/// Runs the read service.
pub fn run(r: &super::command::Read) -> eyre::Result<()> {
    // Parse the ENR
    let enr = r.enr.parse::<Enr>().map_err(|_| eyre::eyre!("Failed to parse enr from string: \"{}\"", r.enr))?;

    // Print the ENR
    common::output::print_enr(enr);

    Ok(())
}