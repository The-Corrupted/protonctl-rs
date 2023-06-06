pub mod github;
pub mod cmd;
pub mod list;
pub mod install;
pub mod remove;
pub mod constants;
pub mod ui;

use cmd::{Actions, ProtonCtl};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let proton = ProtonCtl::parse();
    match &proton.actions {
        Actions::Install(install) => {
            install.run().await?;
        }
        Actions::List(list) => {
            list.run().await?;
        }
        Actions::Remove(remove) => {
            remove.run().await?;
        }
        _ => {}
    }
    Ok(())
}
