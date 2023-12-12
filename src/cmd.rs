use std::fmt::Display;

use async_trait::async_trait;
use clap::{builder::PossibleValue, value_parser, Arg, ArgAction, Command, ValueEnum};
use dirs::home_dir;
use protonctllib::constants;

#[async_trait]
pub trait Run {
    async fn run(&self) -> anyhow::Result<()>;
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub enum InstallTypeCmd {
    Proton,
    Wine,
}

impl Default for InstallTypeCmd {
    fn default() -> Self {
        Self::Proton
    }
}

impl ValueEnum for InstallTypeCmd {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Proton, Self::Wine]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            InstallTypeCmd::Wine => Some(PossibleValue::new("wine")),
            InstallTypeCmd::Proton => Some(PossibleValue::new("proton")),
        }
    }

    fn from_str(input: &str, _ignore_case: bool) -> Result<Self, String> {
        match input {
            "wine" => Ok(Self::Wine),
            "proton" => Ok(Self::Proton),
            _ => Err(format!("Invalid argument: {}", input)),
        }
    }
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

pub async fn build_cli() -> Command {
    Command::new("protonctl")
        .arg(
            Arg::new("type")
                .action(ArgAction::Set)
                .value_parser(clap::builder::EnumValueParser::<InstallTypeCmd>::new())
                .default_value("proton")
                .required(false)
                .help("The type install type to use"),
        )
        .subcommand_precedence_over_arg(true)
        .subcommand(
            Command::new("list")
                .arg(
                    Arg::new("number")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(u8))
                        .default_value("10")
                        .required(false)
                        .short('n')
                        .long("number")
                        .help("The number of releases to list"),
                )
                .arg(
                    Arg::new("page")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(u8))
                        .default_value("1")
                        .short('p')
                        .long("page")
                        .conflicts_with("local")
                        .help("The page number of remote builds to use"),
                )
                .arg(
                    Arg::new("local")
                        .action(ArgAction::SetTrue)
                        .default_value("false")
                        .required(false)
                        .short('l')
                        .long("local")
                        .conflicts_with_all(["number", "page"])
                        .help("List local proton or wine installs"),
                ),
        )
        .subcommand(
            Command::new("remove")
                .arg(
                    Arg::new("cache")
                        .action(ArgAction::SetTrue)
                        .default_value("false")
                        .required(false)
                        .short('c')
                        .long("cache")
                        .conflicts_with("all")
                        .help("Delete artifacts left behind following install failure"),
                )
                .arg(
                    Arg::new("all")
                        .action(ArgAction::SetTrue)
                        .default_value("false")
                        .required(false)
                        .short('a')
                        .long("all")
                        .conflicts_with("cache")
                        .help("Delete all proton or wine installs"),
                )
                .arg(
                    Arg::new("install_version")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String))
                        .required_unless_present_any(["cache", "all"])
                        .help("Install selected version"),
                ),
        )
        .subcommand(Command::new("install").arg(Arg::new("install_version").required(true)))
}
