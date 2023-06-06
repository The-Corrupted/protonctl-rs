use tokio;
use clap::{Args};
use crate::constants;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Remove {
    #[arg(short = 'v', long, required = true)]
    pub version: String
}

impl Remove {
    pub async fn run(&self) -> anyhow::Result<()> {
        remove(self.version.clone()).await?;
        Ok(())
    }
}

pub async fn remove(version: String) -> anyhow::Result<()> {
    let mut compat_path: std::path::PathBuf  = match constants::HOME_DIR.to_owned() {
        Some(path) => path,
        None => {
            return Err(anyhow::anyhow!("Home directory not found"));
        }
    };
    compat_path.push(constants::STEAM_COMPAT_PATH.to_owned());
    compat_path.push(&version);
    if !compat_path.exists() {
        return Err(anyhow::anyhow!("{} does not exist", &version));
    }
    tokio::fs::remove_dir_all(compat_path).await?;
    Ok(())
}
