use crate::constants;
use crate::cmd::{InstallType, Run};
use clap::Args;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Remove {
    #[arg(short = 'v', long, required = true)]
    pub pw_version: String,
}

impl Run for Remove {
    fn run(&self, install_type: InstallType) -> anyhow::Result<()> {
        remove(install_type, self.pw_version.clone())?;
        Ok(())
    }
}

pub fn remove(install_type: InstallType, version: String) -> anyhow::Result<()> {
    let mut compat_path: std::path::PathBuf = match constants::HOME_DIR.to_owned() {
        Some(path) => path,
        None => {
            return Err(anyhow::anyhow!("Home directory not found"));
        }
    };
    let loc = match install_type {
        InstallType::Wine => constants::LUTRIS_RUNNERS_PATH.to_owned(),
        InstallType::Proton => constants::STEAM_COMPAT_PATH.to_owned(),
    };
    compat_path.push(loc);
    compat_path.push(&version);
    if !compat_path.exists() {
        return Err(anyhow::anyhow!("{} does not exist", &version));
    }
    std::fs::remove_dir_all(compat_path)?;
    Ok(())
}
