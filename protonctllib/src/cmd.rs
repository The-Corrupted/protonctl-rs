use crate::install::Install;
use crate::list::List;
use crate::remove::Remove;
use clap::{Parser, Subcommand, ValueEnum};

pub trait Run {
    fn run(&self, install_type: InstallType) -> anyhow::Result<()>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InstallType {
    Proton,
    Wine,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ProtonCtl {
    #[arg(value_enum, default_value_t = InstallType::Proton, required = false)]
    pub install_type: InstallType,
    #[command(subcommand)]
    pub actions: Option<Actions>,
}

#[derive(Subcommand, Debug)]
pub enum Actions {
    Install(Install),
    List(List),
    Remove(Remove),
}
