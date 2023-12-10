use crate::cmd::InstallTypeCmd;
use anyhow::Context;
use clap::Args;
use console::{Style, Term};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use protonctllib::{
    decompress,
    github::api::{
        download_asset, download_asset_to_memory, get_asset_ids, release_version, Release,
    },
    utils,
};
use reqwest::Response;
use std::io::Write;

// We would like install_type to be position AND skippable. You're currently unable to skip it
// so until I figure out why this is, it's technically mandatory.
#[derive(Args, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Install {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(required = true)]
    install_version: String,
}

// Struct containing all the styles we use in the run function.
// It may be worth allowing the user to customize in the future.
pub(crate) struct Styles {
    success_style: Style,
    fail_style: Style,
    prefix_style: Style,
}

impl Styles {
    pub fn new() -> Self {
        Self {
            success_style: Style::new().italic().bold().green(),
            fail_style: Style::new().italic().bold().red(),
            prefix_style: Style::new().white().bold(),
        }
    }
}

/*
 * This really needs to be cleaned up/broken apart.
 * Currently it:
 * 1.) Creates the Term struct and styles
 * 2.) Spawns and joins the download task, handling errors
 * 3.) Creates the progress bar using the Response from the
 * download task.
 * 4.) Creates a new set of tasks to run the download, cloning
 * almost everything.
 * 5.) Checks the sha hash returning an error if it doesn't match
 * 6.) Decompress the tar file, returning an error if it fails.
 * 7.) Remove the download files.
*/

impl Install {
    pub async fn run(&self) -> anyhow::Result<()> {
        // Get terminal and styles setup
        let mut term = Term::stdout();
        let styles = Styles::new();
        // Get information we need to start the download ( install path, download path, assetids )
        let compat_directory: std::path::PathBuf = self
            .install_type
            .get_compat_directory_safe()
            .context("Failed to get compatibility directory")?;
        let url = self.install_type.get_url(false);
        let release: Release = release_version(&url, &self.install_version).await?;
        let (tar_asset, sha_asset) = get_asset_ids(&self.install_type.get_extension(), &release);
        let mut install_path = utils::get_download_directory_safe()?;
        // Create the clones of everything we need
        // Look into how we can do this without creating so many clones. This uses up a lot of
        // memory and is very slow.
        let d_url = url.clone();
        let d_task1 = tokio::spawn(async move {
            install_path.push(&tar_asset.name);
            match download_asset(d_url, &tar_asset).await {
                Ok(res) => handle_install(&install_path, res).await,
                Err(e) => Err(anyhow::anyhow!(format!(
                    "Failed to complete download: {}",
                    e
                ))),
            }
        });
        let d_task2 = tokio::spawn(async move { download_asset_to_memory(url, &sha_asset).await });
        let (res1, res2) = tokio::join!(d_task1, d_task2);
        let tar_path = res1.context("[TAR] Failed to run task")??;
        let sha_string = res2.context("[SHA] Failed to run task")??;

        // Check and make sure the file hash matches the SHA512 file. If it doesn't we shouldn't
        // unpack it. Indicate a failure has occured and
        term.write_fmt(format_args!(
            "{}",
            styles.prefix_style.apply_to("Checking hash ... ")
        ))
        .unwrap();
        match utils::check_sha(&tar_path, &sha_string) {
            Ok(is_match) => {
                if is_match {
                    term.write_fmt(format_args!(
                        "{}",
                        styles.success_style.apply_to("Success\n")
                    ))
                    .unwrap();
                } else {
                    term.write_fmt(format_args!("{}", styles.fail_style.apply_to("Fail\n")))
                        .unwrap();
                    return Err(anyhow::anyhow!("Hash mismatch error!"));
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        // Decompress the file based on the install_type. We may change this later...
        term.write_fmt(format_args!(
            "{}",
            styles.prefix_style.apply_to("Decompressing ... ")
        ))
        .unwrap();
        match self.install_type {
            InstallTypeCmd::Wine => decompress::lzma(&tar_path, &compat_directory)?,
            InstallTypeCmd::Proton => decompress::gunzip(&tar_path, &compat_directory)?,
        }
        // Nothing has failed and we've reached the end. Remove downloaded files and exit
        term.write_fmt(format_args!(
            "{}",
            styles.success_style.apply_to("Success\n")
        ))
        .unwrap();
        term.write_line(format!("{}", styles.prefix_style.apply_to("Removing artifacts")).as_str())
            .unwrap();
        utils::remove_entry(&tar_path)?;
        Ok(())
    }
}
async fn handle_install(
    path: &std::path::PathBuf,
    response: Response,
) -> anyhow::Result<std::path::PathBuf> {
    let content_length = if let Some(c) = response.content_length() {
        c
    } else {
        0
    };

    let pb = ProgressBar::with_draw_target(Some(content_length), ProgressDrawTarget::stderr());
    pb.set_prefix("Downloading:");
    pb.set_style(ProgressStyle::with_template(
        "{prefix:.bold} {bar} {msg:.dim}",
    )?);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .context(format!("Failed to open file: {:?}", path))?;
    let mut total_install = 0;
    let max_hr = indicatif::HumanBytes(content_length);
    let mut stream = response.bytes_stream();
    while let Some(r) = stream.next().await {
        if let Ok(bytes) = r {
            if file.write_all(&bytes).is_ok() {
                let chunk_size = bytes.len() as u64;
                total_install += chunk_size;
                pb.inc(chunk_size);
                pb.set_message(format!(
                    "{}/{}",
                    indicatif::HumanBytes(total_install),
                    max_hr
                ));
            }
        }
    }
    pb.finish();
    Ok(path.to_path_buf())
}
