use crate::{cli::InstallTypeCmd, install, list, remove};
use async_trait::async_trait;
use clap::Command;
use dirs::home_dir;
use protonctllib::constants;
use std::fmt::Display;

#[async_trait]
pub trait Run {
    async fn run(&self) -> anyhow::Result<()>;
}

impl Display for InstallTypeCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallTypeCmd::Wine => write!(f, "wine"),
            InstallTypeCmd::Proton => write!(f, "proton"),
            InstallTypeCmd::ULWGL => write!(f, "ulwgl"),
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
                        constants::GE_PROJECT_OWNER,
                        constants::WINE_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::GE_PROJECT_OWNER,
                        constants::WINE_PROJECT_NAME
                    )
                }
            }
            InstallTypeCmd::Proton => {
                if latest {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases/latest",
                        constants::GE_PROJECT_OWNER,
                        constants::PROTON_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::GE_PROJECT_OWNER,
                        constants::PROTON_PROJECT_NAME
                    )
                }
            }
            InstallTypeCmd::ULWGL => {
                if latest {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases/latest",
                        constants::ULWGL_PROJECT_OWNER,
                        constants::ULWGL_PROJECT_NAME
                    )
                } else {
                    format!(
                        "https://api.github.com/repos/{}/{}/releases",
                        constants::ULWGL_PROJECT_OWNER,
                        constants::ULWGL_PROJECT_NAME
                    )
                }
            }
        }
    }

    pub fn get_compat_directory_safe(&self, is_flatpak: bool) -> anyhow::Result<std::path::PathBuf> {
        let mut compat_dir =
            home_dir().ok_or(anyhow::anyhow!("Failed to get users home directory"))?;
        let compat_path = match self {
            InstallTypeCmd::Wine => {
                if is_flatpak {
                    std::path::PathBuf::from(".var/app/net.lutris.Lutris/data/lutris/runners/wine")
                } else {
                    std::path::PathBuf::from(".local/share/lutris/runners/wine")
                }
            }
            InstallTypeCmd::Proton => {
                if is_flatpak {
                    std::path::PathBuf::from(".var/app/com.valvesoftware.Steam/.local/share/Steam/compatibilitytools.d")
                } else {
                    std::path::PathBuf::from(".local/share/Steam/compatibilitytools.d")
                }
            },
            InstallTypeCmd::ULWGL => {
                std::path::PathBuf::from(".local/share/ULWGL-Proton/")
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
    let flatpak = *matches.get_one::<bool>("flatpak").unwrap();
    let install_type = *matches.get_one::<InstallTypeCmd>("type").unwrap();
    match matches.subcommand() {
        Some(("install", sub_i)) => Ok(Box::new(install::Install::new(
            sub_i.get_one::<String>("install_version").unwrap().clone(),
            flatpak,
            *sub_i.get_one::<bool>("skip_sha_check").unwrap(),
            install_type,
        ))),
        Some(("list", sub_l)) => Ok(Box::new(list::List::new(
            *sub_l.get_one::<u8>("number").unwrap(),
            *sub_l.get_one::<u8>("page").unwrap(),
            *sub_l.get_one::<bool>("local").unwrap(),
            flatpak,
            install_type,
        ))),
        Some(("remove", sub_r)) => {
            let cache = *sub_r.get_one::<bool>("cache").unwrap();
            let all = *sub_r.get_one::<bool>("all").unwrap();
            let install_version = if let Some(v) = sub_r.get_one::<String>("install_version") {
                v.clone()
            } else {
                if !all && !cache {
                    return Err(anyhow::anyhow!("No install_version specified"));
                }
                String::new()
            };

            Ok(Box::new(remove::Remove::new(
                cache,
                all,
                flatpak,
                install_type,
                install_version,
            )))
        }
        _ => {
            Err(anyhow::anyhow!("It shouldn't be possible to hit this"))
        }
    }
}
