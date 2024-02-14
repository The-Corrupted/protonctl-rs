use clap::{builder::PossibleValue, value_parser, Arg, ArgAction, Command, ValueEnum};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub enum InstallTypeCmd {
    Proton,
    Wine,
    ULWGL,
}

impl Default for InstallTypeCmd {
    fn default() -> Self {
        Self::Proton
    }
}

impl ValueEnum for InstallTypeCmd {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Proton, Self::Wine, Self::ULWGL]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            InstallTypeCmd::Wine => Some(PossibleValue::new("wine")),
            InstallTypeCmd::Proton => Some(PossibleValue::new("proton")),
            InstallTypeCmd::ULWGL => Some(PossibleValue::new("ulwgl")),
        }
    }

    fn from_str(input: &str, _ignore_case: bool) -> Result<Self, String> {
        match input {
            "wine" => Ok(Self::Wine),
            "proton" => Ok(Self::Proton),
            "ulwgl" => Ok(Self::ULWGL),
            _ => Err(format!("Invalid argument: {}", input)),
        }
    }
}

pub fn build_cli() -> Command {
    Command::new("protonctl")
        .subcommand_precedence_over_arg(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("list")
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .action(ArgAction::Set)
                        .value_parser(clap::builder::EnumValueParser::<InstallTypeCmd>::new())
                        .default_value("proton")
                        .global(true)
                        .required(false)
                        .help("The type install type to use"),
                )
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
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .action(ArgAction::Set)
                        .value_parser(clap::builder::EnumValueParser::<InstallTypeCmd>::new())
                        .default_value("proton")
                        .global(true)
                        .required(false)
                        .help("The type install type to use"),
                )
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
        .subcommand(
            Command::new("install")
                .arg(Arg::new("install_version").required(true))
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .action(ArgAction::Set)
                        .value_parser(clap::builder::EnumValueParser::<InstallTypeCmd>::new())
                        .default_value("proton")
                        .global(true)
                        .required(false)
                        .help("The type install type to use"),
                ),
        )
}
