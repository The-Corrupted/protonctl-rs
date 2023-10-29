
use crate::github::api::{download_assets, get_asset_ids, release_version, Release};
use crate::os_helper::get_compat_directory_safe;


use clap::Args;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    #[arg(short = 'p', long = "proton-version")]
    pub proton_version: String,
}

impl Install {
    pub fn run(&self) -> anyhow::Result<()> {
        let _compat_directory: std::path::PathBuf = get_compat_directory_safe()?;
        let release: Release = release_version(self.proton_version.clone())?;
        let assets = get_asset_ids(release)?;
        download_assets(assets)?;
        Ok(())
    }
}
