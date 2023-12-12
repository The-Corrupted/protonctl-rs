use crate::cli::InstallTypeCmd;
use crate::cli_utils::Run;
use async_trait::async_trait;
use protonctllib::{utils, version_info};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Remove {
    pub cache: bool,
    pub all: bool,
    pub install_type: InstallTypeCmd,
    pub pw_version: std::path::PathBuf,
}

impl Remove {
    pub fn new(
        cache: bool,
        all: bool,
        install_type: InstallTypeCmd,
        pw_version: std::path::PathBuf,
    ) -> Self {
        Self {
            cache,
            all,
            install_type,
            pw_version,
        }
    }

    pub fn set_cache(&mut self, cache: bool) -> &mut Self {
        self.cache = cache;
        self
    }

    pub fn set_all(&mut self, all: bool) -> &mut Self {
        self.all = all;
        self
    }

    pub fn set_install_type(&mut self, install_type: InstallTypeCmd) -> &mut Self {
        self.install_type = install_type;
        self
    }

    pub fn set_pw_version(&mut self, pw_version: std::path::PathBuf) -> &mut Self {
        self.pw_version = pw_version;
        self
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
