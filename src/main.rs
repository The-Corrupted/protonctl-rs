pub mod cmd;
pub mod install;
pub mod list;
pub mod remove;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cmd::build_cli();
    let runner = cmd::command_to_struct(&cmd)?;
    runner.run().await?;
    Ok(())
}
