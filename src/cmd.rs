use clap::{Args, Parser, Subcommand};

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
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    #[arg(short = 'i', long, conflicts_with = "proton_version")]
    pub interactive: bool,
    #[arg(short = 'p', long = "proton-version", conflicts_with = "interactive")]
    pub proton_version: String,
}

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct List {
    #[arg(short = 'n', long, default_value_t = 10, required = false)]
    pub number: u8,
    #[arg(short = 'b', long = "body", required = false)]
    pub get_body: bool
}
