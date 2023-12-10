pub mod cmd;
pub mod install;
pub mod list;
pub mod remove;

use crate::cmd::Actions;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parser: cmd::ProtonCtl = Parser::parse();
    if let Some(actions) = parser.actions {
        match actions {
            Actions::Install(i) => i.run().await?,
            Actions::List(l) => l.run().await?,
            Actions::Remove(r) => r.run().await?,
        }
    }
    Ok(())
}
