use clap::{Args};

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    #[arg(short = 'i', long, conflicts_with = "proton_version")]
    pub interactive: bool,
    #[arg(short = 'p', long = "proton-version", conflicts_with = "interactive")]
    pub proton_version: String,
}
