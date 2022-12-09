use std::{fs::File, io::Read};

use enr::CombinedKey;

/// Reads a private key from a file.
pub fn read_from_file(path: &str) -> eyre::Result<Vec<u8>> {
    let mut file = File::open(path).map_err(|_| "Cannot find key-file").map_err(|e| eyre::eyre!(e))?;
    let mut key_bytes: Vec<u8> = Vec::with_capacity(36);
    file.read_to_end(&mut key_bytes)
        .map_err(|_| eyre::eyre!("Cannot read key file \"{}\"", path))?;
    Ok(key_bytes)
}

/// Generates a private key from bytes.
pub fn generate(bytes: Option<&mut [u8]>) -> eyre::Result<CombinedKey> {
    bytes.map(|b|
        CombinedKey::secp256k1_from_bytes(b)
            .or_else(|_| CombinedKey::ed25519_from_bytes(b))
            .map_err(|_| eyre::eyre!("Invalid private key"))
    )
    .or_else(|| Some(Ok::<CombinedKey, eyre::Report>(CombinedKey::generate_secp256k1())))
    .ok_or(eyre::eyre!("Invalid private key"))?
}