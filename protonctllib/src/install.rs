
use std::io::Read;

use crate::github::api::{download_assets, get_asset_ids, release_version, Release};
use crate::cmd::{Run, InstallType};
use crate::os_helper::get_compat_directory_safe;

use anyhow::{Context};
use clap::Args;
use crate::decompress;

use sha2::{Digest, Sha512};


#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    install_version: String,
}

impl Run for Install {
    fn run(&self, install_type: InstallType) -> anyhow::Result<()> {
        let compat_directory: std::path::PathBuf = get_compat_directory_safe(install_type)?;
        let release: Release = release_version(install_type, self.install_version.clone())?;
        let assets = get_asset_ids(install_type, release)?;
        let downloaded = download_assets(install_type, assets)?;
        self.check_sha(&downloaded)?;
        match install_type {
            InstallType::Wine => decompress::lmza(downloaded[0].clone(), compat_directory)?,
            InstallType::Proton => decompress::gunzip(downloaded[0].clone(), compat_directory)?
        }
        Ok(())
    }
}

impl Install {

    pub fn check_sha(&self, file_locations: &[std::path::PathBuf; 2]) -> anyhow::Result<()> {
        println!("Checking hash");
        // Open the tar file and pass it into the hasher
        let mut file_location = file_locations[0].clone();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(&file_location)
            .context(format!("Failed to open compressed file: {:?}", file_location))?;
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
        file = std::fs::OpenOptions::new()
            .read(true)
            .open(&file_location)
            .context(format!("Failed to open sha file: {:?}", file_location))?;
        let mut expected_hash = String::new();
        file.read_to_string(&mut expected_hash)
            .context("Failed to read file into string")?;
        expected_hash = expected_hash.get(0..128)
            .context("Failed to hash value")?
            .into();
        if expected_hash == hash {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Hash mismatch\n{}\n{}", expected_hash, hash))
        }
    }
}
