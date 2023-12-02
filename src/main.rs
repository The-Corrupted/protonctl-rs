use clap::Parser;
use protonctllib::cmd::{Actions, ProtonCtl};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let proton = ProtonCtl::parse();
    if let Some(subcommand) = proton.actions {
        match subcommand {
            Actions::Install(install) => {
                install.run(proton.install_type).await?;
            }
            Actions::List(list) => {
                list.run(proton.install_type).await?;
            }
            Actions::Remove(remove) => {
                remove.run(proton.install_type).await?;
            }
        }
    }
    Ok(())
}
