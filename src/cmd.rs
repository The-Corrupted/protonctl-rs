use std::fmt::Display;

use crate::install::Install;
use crate::list::List;
use crate::remove::Remove;
use clap::{Parser, Subcommand, ValueEnum};
use protonctllib::constants;
use dirs::home_dir;


#[derive(Eq, PartialEq, Ord, PartialOrd, ValueEnum, Clone, Copy, Debug)]
pub enum InstallTypeCmd {
    Proton,
    Wine,
}

impl Display for InstallTypeCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallTypeCmd::Wine => write!(f, "wine"),
            InstallTypeCmd::Proton => write!(f, "proton")
        }
    }
}

impl InstallTypeCmd {
    pub fn get_url(&self, latest: bool) -> String {
       let mut url: String = String::new();
        match self {
            InstallTypeCmd::Wine => {
                url = if latest {
                    format!("https://api.github.com/repos/{}/{}/releases/latest",
                            constants::PROJECT_OWNER, constants::WINE_PROJECT_NAME)
                } else {

                    format!("https://api.github.com/repos/{}/{}/releases",
                              constants::PROJECT_OWNER, constants::WINE_PROJECT_NAME)
                }
            },
            InstallTypeCmd::Proton => {
                url = if latest {
                    format!("https://api.github.com/repos/{}/{}/releases/latest",
                           constants::PROJECT_OWNER, constants::PROTON_PROJECT_NAME)
                } else {
                    format!("https://api.github.com/repos/{}/{}/releases",
                            constants::PROJECT_OWNER, constants::PROTON_PROJECT_NAME)
                }
            }
        }
        url

    }

    pub fn get_extension(&self) -> String {
        match self {
            InstallTypeCmd::Wine => String::from(".tar.xz"),
            InstallTypeCmd::Proton => String::from(".tar.gz")
        }
    }

    pub fn get_compat_directory_safe(&self) -> anyhow::Result<std::path::PathBuf> {
        let mut compat_dir = home_dir().ok_or(anyhow::anyhow!("Failed to get users home directory"))?;

        let compat_path = match self {
            InstallTypeCmd::Wine => std::path::PathBuf::from(".local/share/lutris/runners/wine"),
            InstallTypeCmd::Proton => std::path::PathBuf::from(".local/share/Steam/compatibilitytools.d"),
        };
        compat_dir.push(compat_path);
        if !compat_dir.exists() {
            std::fs::create_dir_all(&compat_dir)?;
            Ok(compat_dir)
        } else {
            Ok(compat_dir)
        }
    }
}

#[derive(Parser, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ProtonCtl {
    #[command(subcommand)]
    pub actions: Option<Actions>,
}

#[derive(Subcommand, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Actions {
    Install(Install),
    List(List),
    Remove(Remove),
}
