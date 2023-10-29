
use std::io::Read;

use crate::github::api::{download_assets, get_asset_ids, release_version, Release};
use crate::os_helper::get_compat_directory_safe;


use clap::Args;
use flate2::read::GzDecoder;
use sha2::{Digest, Sha512};
use tar::Archive;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    #[arg(short = 'p', long = "proton-version")]
    pub proton_version: String,
}

impl Install {
    pub fn run(&self) -> anyhow::Result<()> {
        let compat_directory: std::path::PathBuf = get_compat_directory_safe()?;
        let release: Release = release_version(self.proton_version.clone())?;
        let assets = get_asset_ids(release)?;
        let downloaded = download_assets(assets)?;
        self.check_sha(&downloaded)?;
        self.unpack_file(downloaded[0].clone(),compat_directory)?;
        Ok(())
    }

    pub fn unpack_file(&self, compressed_path: std::path::PathBuf, output_path: std::path::PathBuf) -> anyhow::Result<()> {
        println!("Unpacking file");
        let file = match std::fs::OpenOptions::new()
            .read(true)
            .open(compressed_path) {
                Ok(f) => f,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to open compressed file for reading: {:?}", e));
                },
            };
        let mut archive = Archive::new(GzDecoder::new(file));
        match archive.unpack(output_path) {
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to decompress archive: {:?}", e));
            }
            _ => Ok(())
        }
    }

    pub fn check_sha(&self, file_locations: &[std::path::PathBuf; 2]) -> anyhow::Result<()> {
        println!("Checking hash");
        // Open the tar file and pass it into the hasher
        let mut file_location = file_locations[0].clone();
        let mut file = match std::fs::OpenOptions::new()
            .read(true)
            .open(file_location) {
                Ok(e) => e,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to open file: {:?}", e));
                }
            };
        let mut hasher = Sha512::new();
        match std::io::copy(&mut file, &mut hasher) {
            Err(e) => {
                println!("Failed to copy file contents to hasher: {:?}", e);
            },
            _ => (),
        }
        let hash = format!("{:x}", hasher.finalize());
        // Get the expected hash from the downloaded sha file.
        file_location = file_locations[1].clone();
        file = match std::fs::OpenOptions::new()
            .read(true)
            .open(file_location) {
                Ok(f) => f,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to open the sha file: {:?}", e));
                }
            };
        let mut expected_hash = String::new();
        match file.read_to_string(&mut expected_hash) {
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to read file into string: {:?}", e));
            },
            _ => (),
        }
        expected_hash = match expected_hash.get(0..128) {
            Some(e) => {
                e.into()
            },
            None => {
                return Err(anyhow::anyhow!("Failed to extract the hash"));
            }
        };
        if expected_hash == hash {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Hash mismatch\n{}\n{}", expected_hash, hash))
        }
    }
}
