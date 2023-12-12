pub mod cmd;
pub mod install;
pub mod list;
pub mod remove;

use crate::cmd::Run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = cmd::build_cli().await;
    let mut matches = cmd.get_matches();
    let itype: cmd::InstallTypeCmd = matches.remove_one::<cmd::InstallTypeCmd>("type").unwrap();

    match matches.subcommand() {
        Some(("install", sub_i)) => {
            install::Install::new(
                sub_i.get_one::<String>("install_version").unwrap().clone(),
                itype,
            )
            .run()
            .await?;
        }
        Some(("list", sub_l)) => {
            list::List::new(
                *sub_l.get_one::<u8>("number").unwrap(),
                *sub_l.get_one::<u8>("page").unwrap(),
                *sub_l.get_one::<bool>("local").unwrap(),
                itype,
            )
            .run()
            .await?;
        }
        Some(("remove", sub_r)) => {
            let pw_version = if let Some(v) = sub_r.get_one::<std::path::PathBuf>("pw_version") {
                v.clone()
            } else {
                std::path::PathBuf::new()
            };

            remove::Remove::new(
                *sub_r.get_one::<bool>("cache").unwrap(),
                *sub_r.get_one::<bool>("all").unwrap(),
                itype,
                pw_version,
            )
            .run()
            .await?;
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Failed to get subcommand. This shouldn't be possible"
            ))
        }
    }

    Ok(())
}
