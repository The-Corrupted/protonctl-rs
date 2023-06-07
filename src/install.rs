use async_compression::tokio::bufread::GzipDecoder;
use clap::{Args};
use reqwest;
use tokio;
use crate::constants;
use crate::github::api::{Release, get_asset_ids, 
    download_assets, release_version};

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    #[arg(short = 'p', long = "proton-version", conflicts_with = "interactive")]
    pub proton_version: String,
}

impl Install {
    pub async fn run(&self) -> anyhow::Result<()> {
        let compat_directory: std::path::PathBuf = get_compat_directory_safe().await?;
        let install_directory: std::path::PathBuf = get_install_directory_safe().await?;
        let release: Release = release_version(self.proton_version.clone()).await?;
        let assets = get_asset_ids(release).await?;
        Ok(())
    }
}

pub async fn get_compat_directory_safe() -> anyhow::Result<std::path::PathBuf> {
    let mut compat_dir = match constants::HOME_DIR.to_owned() {
        Some(home) => home,
        None => {
            return Err(anyhow::anyhow!("Could not find users home directory"));
        }
    };
    compat_dir.push(constants::STEAM_COMPAT_PATH.clone());
    if !compat_dir.exists() {
        tokio::fs::create_dir_all(&compat_dir).await?;
        println!("Created compatibility tools directory");
        Ok(compat_dir)
    } else {
        Ok(compat_dir)
    }
}

pub async fn get_install_directory_safe() -> anyhow::Result<std::path::PathBuf> {
    let mut install_dir = match constants::HOME_DIR.to_owned() {
        Some(home) => home,
        None => {
            return Err(anyhow::anyhow!("Could not find users home directory"));
        }
    };
    install_dir.push(constants::INSTALL_PATH.clone());
    if !install_dir.exists() {
        tokio::fs::create_dir_all(&install_dir).await?;
        println!("Create shared install directory");
        Ok(install_dir)
    } else {
        Ok(install_dir)
    }
}

// pub async fn install_files(
