use std::fmt::Display;
use crate::{install, remove, list, cli::InstallTypeCmd};
use async_trait::async_trait;
use clap::Command;
use dirs::home_dir;
use protonctllib::constants;

#[async_trait]
pub trait Run {
    async fn run(&self) -> anyhow::Result<()>;
}

impl Display for InstallTypeCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallTypeCmd::Wine => write!(f, "wine"),
            InstallTypeCmd::Proton => write!(f, "proton"),
        }
    }
}

impl InstallTypeCmd {
    pub fn get_url(&self, latest: bool) -> String {
        match self {
            InstallTypeCmd::Wine => {
                if latest {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases/latest",
                        constants::PROJECT_OWNER,
                        constants::WINE_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::PROJECT_OWNER,
                        constants::WINE_PROJECT_NAME
                    )
                }
            }
            InstallTypeCmd::Proton => {
                if latest {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases/latest",
                        constants::PROJECT_OWNER,
                        constants::PROTON_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::PROJECT_OWNER,
                        constants::PROTON_PROJECT_NAME
                    )
                }
            }
        }
    }

    pub fn get_compat_directory_safe(&self) -> anyhow::Result<std::path::PathBuf> {
        let mut compat_dir =
            home_dir().ok_or(anyhow::anyhow!("Failed to get users home directory"))?;

        let compat_path = match self {
            InstallTypeCmd::Wine => std::path::PathBuf::from(".local/share/lutris/runners/wine"),
            InstallTypeCmd::Proton => {
                std::path::PathBuf::from(".local/share/Steam/compatibilitytools.d")
            }
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

pub fn command_to_struct(cmd: &Command) -> anyhow::Result<Box<dyn Run>> {
    let matches = cmd.clone().get_matches();
    match matches.subcommand() {
        Some(("install", sub_i)) => {
            Ok(Box::new(install::Install::new(
                sub_i.get_one::<String>("install_version").unwrap().clone(),
                *sub_i.get_one::<InstallTypeCmd>("type").unwrap(),
            )))
        }
        Some(("list", sub_l)) => {
            Ok(Box::new(list::List::new(
                *sub_l.get_one::<u8>("number").unwrap(),
                *sub_l.get_one::<u8>("page").unwrap(),
                *sub_l.get_one::<bool>("local").unwrap(),
                *sub_l.get_one::<InstallTypeCmd>("type").unwrap(),
            )))
        }
        Some(("remove", sub_r)) => {
            let cache = *sub_r.get_one::<bool>("cache").unwrap();
            let all = *sub_r.get_one::<bool>("all").unwrap();
            let install_version = if let Some(v) = sub_r.get_one::<String>("install_version") {
                v.clone()
            } else {
                if !all && !cache {
                    return Err(anyhow::anyhow!("No install_version specified"))
                }
                String::new()
            };

            Ok(Box::new(remove::Remove::new(
                cache,
                all,
                *sub_r.get_one::<InstallTypeCmd>("type").unwrap(),
                install_version
            )))
        }
        _ => { return Err(anyhow::anyhow!("It shouldn't be possible to hit this")); }
    }
}
