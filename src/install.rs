use clap::{Args};
use reqwest;
use tokio;
use crate::constants;
use crate::github::api::{get_release_asset_ids, download_release_assets,
    release_version};

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    #[arg(short = 'p', long = "proton-version", conflicts_with = "interactive")]
    pub proton_version: String,
}

impl Install {
    pub async fn run(&self) -> anyhow::Result<()> {
        let compat_directory = get_compat_directory_safe(true).await?;
        let release = release_version(self.proton_version.clone()).await?;
        let asset_ids = get_release_asset_ids(release).await?;
        
        Ok(())
    }
}

pub async fn get_compat_directory_safe(should_create: bool) -> anyhow::Result<std::path::PathBuf> {
    let mut compat_dir = match constants::HOME_DIR.to_owned() {
        Some(home) => home,
        None => {
            return Err(anyhow::anyhow!("Could not find users home directory"));
        }
    };
    compat_dir.push(constants::STEAM_COMPAT_PATH.clone());
    if !compat_dir.exists() {
        if should_create {
            tokio::fs::create_dir_all(&compat_dir).await?;
            println!("Created compatibility tools directory");
            Ok(compat_dir)
        } else {
            Err(anyhow::anyhow!("Compat directory doesn't exist and couldn't be made"))
        }
    } else {
        println!("Compatibility tools directory exists");
        Ok(compat_dir)
    }
}

// pub async fn get_tarball(tarball_url)
