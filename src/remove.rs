use crate::cmd::InstallTypeCmd;
use clap::Args;
use protonctllib::{utils, version_info};

#[derive(Args, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub struct Remove {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(short='c', long="cache", required = false, conflicts_with_all = &["all", "pw_version"], default_value_t = false)]
    cache: bool,
    #[arg(short='a', long="all", required = false, conflicts_with_all = &["cache", "pw_version"], default_value_t = false)]
    all: bool,
    #[arg(required_unless_present_any = &["all", "cache"], conflicts_with_all = &["cache", "all"],)]
    pub pw_version: Option<std::path::PathBuf>,
}

impl Remove {
    pub async fn run(&self) -> anyhow::Result<()> {
        if self.all {
            let compat_path = self.install_type.get_compat_directory_safe()?;
            utils::remove_all_in(&compat_path)?;
        } else if self.cache {
            let install_path = utils::get_download_directory_safe()?;
            utils::remove_all_in(&install_path)?;
        } else {
            let mut compat_path = self.install_type.get_compat_directory_safe()?;
            let installed_versions = version_info::get_installed_versions(&compat_path)?;
            compat_path.push(&self.pw_version.clone().unwrap());
            if let Some(item) = installed_versions
                .into_iter()
                .find(|e| e.path() == compat_path)
            {
                utils::remove_entry(&item.path())?;
            } else {
                eprintln!("{:?} not found", self.pw_version);
            }
        }
        Ok(())
    }
}
