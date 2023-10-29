use crate::install::Install;
use crate::list::List;
use crate::remove::Remove;
use clap::{Parser, Subcommand};

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ProtonCtl {
    #[command(subcommand)]
    pub actions: Option<Actions>,
}

#[derive(Subcommand, Debug)]
pub enum Actions {
    Install(Install),
    List(List),
    Remove(Remove),
}
