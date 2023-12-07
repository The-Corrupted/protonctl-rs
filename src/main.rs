pub mod list;
pub mod cmd;
pub mod install;
pub mod remove;

use clap::Parser;
use crate::cmd::Actions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parser: cmd::ProtonCtl = Parser::parse();
    if let Some(actions) = parser.actions {
        match actions {
            Actions::Install(i) => i.run().await?,
            Actions::List(l) => l.run().await?,
            Actions::Remove(r) => r.run().await?
        }
    }
    Ok(())
}
