use crate::cli::InstallTypeCmd;
use crate::cli_utils::Run;
use anyhow::Context;
use async_trait::async_trait;
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Install {
    pub install_version: String,
    pub flatpak: bool,
    pub install_type: InstallTypeCmd,
}

impl Install {
    pub fn new(install_version: String, flatpak: bool, install_type: InstallTypeCmd) -> Self {
        Self {
            install_version,
            flatpak,
            install_type,
        }
    }
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

#[async_trait]
impl Run for Install {
    async fn run(&self) -> anyhow::Result<()> {
        // Get terminal and styles setup
        let mut term = Term::stderr();
        let styles = Styles::new();
        // Get information we need to start the download ( install path, download path, assetids )
        let compat_directory: std::path::PathBuf = self
            .install_type
            .get_compat_directory_safe(self.flatpak)
            .context("Failed to get compatibility directory")?;
        let url = self.install_type.get_url(false);
        let release: Release = release_version(&url, &self.install_version).await?;
        let (tar_asset, sha_asset) = get_asset_ids(&release);
        let mut install_path = utils::get_download_directory_safe()?;
        install_path.push(&tar_asset.name);

        let tar_path = handle_install(
            &install_path,
            download_asset(url.clone(), &tar_asset).await?,
        )
        .await?;
        let sha_string = download_asset_to_memory(url, &sha_asset).await?;
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

        decompress::decompress(&tar_path, &compat_directory)?;

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
        "{prefix:.bold} {wide_bar} {msg:.dim}",
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
