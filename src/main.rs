use clap::Parser;
use protonctllib::cmd::{Actions, ProtonCtl, Run};

fn main() -> anyhow::Result<()> {
    let proton = ProtonCtl::parse();
    if let Some(subcommand) = proton.actions {
        match subcommand {
            Actions::Install(install) => {
                install.run(proton.install_type)?;
            }
            Actions::List(list) => {
                list.run(proton.install_type)?;
            }
            Actions::Remove(remove) => {
                remove.run(proton.install_type)?;
            }
        }
    }
    Ok(())
}
