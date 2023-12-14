use crate::cli::InstallTypeCmd;
use crate::cli_utils::Run;
use async_trait::async_trait;
use protonctllib::{utils, version_info};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Remove {
    pub cache: bool,
    pub all: bool,
    pub install_type: InstallTypeCmd,
    pub pw_version: String,
}

impl Remove {
    pub fn new(
        cache: bool,
        all: bool,
        install_type: InstallTypeCmd,
        pw_version: String,
    ) -> Self {
        Self {
            cache,
            all,
            install_type,
            pw_version,
        }
    }
}

#[async_trait]
impl Run for Remove {
    async fn run(&self) -> anyhow::Result<()> {
        if self.all {
            let compat_path = self.install_type.get_compat_directory_safe()?;
            utils::remove_all_in(&compat_path)?;
        } else if self.cache {
            let install_path = utils::get_download_directory_safe()?;
            utils::remove_all_in(&install_path)?;
        } else {
            let mut compat_path = self.install_type.get_compat_directory_safe()?;
            let installed_versions = version_info::get_installed_versions(&compat_path)?;
            compat_path.push(&self.pw_version);
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
