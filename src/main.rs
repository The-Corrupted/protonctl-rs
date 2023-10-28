pub mod cmd;
pub mod constants;
pub mod github;
pub mod install;
pub mod list;
pub mod remove;

use clap::Parser;
use cmd::{Actions, ProtonCtl};

fn main() -> anyhow::Result<()> {
    let proton = ProtonCtl::parse();
    if let Some(subcommand) = proton.actions {
        match subcommand {
            Actions::Install(install) => {
                install.run()?;
            }
            Actions::List(list) => {
                list.run()?;
            }
            Actions::Remove(remove) => {
                remove.run()?;
            }
        }
    }
    Ok(())
}
