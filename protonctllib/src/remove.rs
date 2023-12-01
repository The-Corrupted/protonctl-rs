use crate::constants::{LockReferences, paths};
use crate::cmd::{InstallType, Run};
use crate::list::get_installed_versions;
use crate::os_helper::{remove_entry, remove_all_in};

use clap::Args;
use dirs::home_dir;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Remove {
    #[arg(short = 'c', long = "cache", required = false, conflicts_with_all = ["pw_version", "all"], default_value_t = false)]
    cache: bool,
    #[arg(short = 'a', long = "all", required = false, conflicts_with_all = ["pw_version", "cache"], default_value_t = false)]
    all: bool,
    #[arg(required_unless_present_any = ["all", "cache"], conflicts_with_all = ["all", "cache"])]
    pub pw_version: std::path::PathBuf,
}

impl Run for Remove {
    fn run(&self, install_type: InstallType) -> anyhow::Result<()> {
        let mut compat_path: std::path::PathBuf = home_dir()
            .ok_or(anyhow::anyhow!("Failed to get users home directory"))?;
        let loc = match install_type {
            InstallType::Wine => paths()
                .get(&LockReferences::LutrisRunnersPath)
                .unwrap(),
            InstallType::Proton => paths()
                .get(&LockReferences::SteamCompatPath)
                .unwrap(),
        };
        compat_path.push(loc);
        if self.all {
            remove_all_in(&compat_path)?;
        } else if self.cache {
            compat_path.push(paths()
                             .get(&LockReferences::InstallPath)
                             .unwrap());
            remove_all_in(&compat_path)?;
        } else {
            let installed_versions = get_installed_versions(install_type)?;
            compat_path.push(&self.pw_version);
            if let Some(item) = installed_versions.into_iter().find(|e| e.path() == compat_path) {
                remove_entry(&item.path())?;
            } else {
                println!("{:?} not found", self.pw_version);
            }
        }
        Ok(())
    }
}