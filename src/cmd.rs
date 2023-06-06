use clap::{Args, Parser, Subcommand};
use crate::install::Install;
use crate::remove::Remove;
use crate::list::List;

pub trait Run {
    fn run(&self) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ProtonCtl {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Subcommand, Debug)]
pub enum Actions {
    Install(Install),
    List(List),
    Remove(Remove),
}


