use clap::Parser;
use protonctllib::cmd::{Actions, ProtonCtl};

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
