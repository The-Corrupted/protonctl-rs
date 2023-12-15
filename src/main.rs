pub mod cli;
pub mod cli_utils;
pub mod install;
pub mod list;
pub mod remove;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cli::build_cli();
    let runner = cli_utils::command_to_struct(&cmd)?;
    runner.run().await?;
    Ok(())
}
