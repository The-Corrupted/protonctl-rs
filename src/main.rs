pub mod github;
pub mod cmd;
pub mod list;
pub mod install;
pub mod remove;
pub mod constants;

use cmd::{Actions, ProtonCtl};
use clap::Parser;

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
