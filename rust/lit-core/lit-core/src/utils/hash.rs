use std::fs::File;
use std::io;
use std::path::Path;

use sha2::digest::Output;
use sha2::{Digest, Sha256, Sha384, Sha512};

use crate::error::{io_err, Result};

pub fn sha512_file(file_path: &Path) -> Result<Output<Sha512>> {
    let mut hasher = Sha512::new();
    let mut file = File::open(file_path).map_err(|e| io_err(e, None))?;

    let _ = io::copy(&mut file, &mut hasher).map_err(|e| io_err(e, None))?;
    Ok(hasher.finalize())
}

pub fn sha512<B>(data: B) -> Output<Sha512>
where
    B: AsRef<[u8]>,
{
    let mut hasher = Sha512::new();
    hasher.update(data.as_ref());
    hasher.finalize()
}

pub fn sha384_file(file_path: &Path) -> Result<Output<Sha384>> {
    let mut hasher = Sha384::new();
    let mut file = File::open(file_path).map_err(|e| io_err(e, None))?;

    let _ = io::copy(&mut file, &mut hasher).map_err(|e| io_err(e, None))?;
    Ok(hasher.finalize())
}

pub fn sha384<B>(data: B) -> Output<Sha384>
where
    B: AsRef<[u8]>,
{
    let mut hasher = Sha384::new();
    hasher.update(data.as_ref());
    hasher.finalize()
}

pub fn sha256_file(file_path: &Path) -> Result<Output<Sha256>> {
    let mut hasher = Sha256::new();
    let mut file = File::open(file_path).map_err(|e| io_err(e, None))?;

    let _ = io::copy(&mut file, &mut hasher).map_err(|e| io_err(e, None))?;
    Ok(hasher.finalize())
}

pub fn sha256<B>(data: B) -> Output<Sha256>
where
    B: AsRef<[u8]>,
{
    let mut hasher = Sha256::new();
    hasher.update(data.as_ref());
    hasher.finalize()
}
