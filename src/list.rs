use clap::Args;
use anyhow::Context;
use crate::cmd::InstallTypeCmd;
use protonctllib::version_info::{get_releases_paged, get_installed_versions};

#[derive(Args, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct List {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(long = "number", short = 'n', required = false, default_value_t = 10)]
    pub number: u8,
    #[arg(long = "page", short = 'p', required = false, default_value_t = 1)]
    pub page: u8,
    #[arg(short = 'l', required = false, default_value_t = false)]
    pub local: bool,
}

// We need to do output stuff here.
impl List {
    pub async fn run(&self) -> anyhow::Result<()> {
        if self.local {
            let mut iters = 0;
            let versions = get_installed_versions(self.install_type.get_compat_directory_safe()
                                                  .context("Failed to get compatibility directory")?)
                .context("Failed to get directory entries")?;
            for version in versions {
                let version = version.file_name();
                if let Some(name) = version.to_str() {
                    let mut name = name.to_string();
                    name.push_str("   ");
                } else {
                    eprintln!("Failed to convert file_name to string");
                }
                iters += 1;
            }
        } else if let Some(releases) = get_releases_paged(self.install_type.get_url(false), self.number, self.page).await {
            for release in releases {
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get releases"));
        }
        Ok(())
    }
}
