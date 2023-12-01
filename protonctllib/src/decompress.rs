use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::path::PathBuf;
use tar::Archive;
use xz2::read::XzDecoder;

pub fn gunzip(compressed: std::path::PathBuf, out: PathBuf) -> Result<()> {
    println!("Unpacking gunzip compressed file");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(compressed)
        .context("Failed to open compressed file for reading")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    archive
        .unpack(out)
        .context("Failed to unpack gunzip file")?;
    Ok(())
}

pub fn lzma(compressed: std::path::PathBuf, out: PathBuf) -> Result<()> {
    println!("Unpacking lzma compressed file");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(compressed)
        .context("Failed to open compressed file for reading")?;
    let mut archive = Archive::new(XzDecoder::new(file));
    archive.unpack(out).context("Failed to unpack xz file")?;
    Ok(())
}
