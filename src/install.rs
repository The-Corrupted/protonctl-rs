use clap::Args;
use std::io::Write;
use reqwest::Response;
use anyhow::Context;
use console::{Term, Style, Color};
use crate::cmd::InstallTypeCmd;
use protonctllib::{utils, decompress, github::api::{release_version, get_asset_ids, download_asset, Release}};

#[derive(Args, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Install {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(required = true)]
    install_version: String,
}

impl Install {
    pub async fn run(&self) -> anyhow::Result<()> {
        // Get terminal and styles setup
        let mut term = Term::stdout();
        let success_style = Style::new().italic().bold().green();
        let fail_style = Style::new().italic().bold().red();
        let prefix_style = Style::new().white().bold();
        let compat_directory: std::path::PathBuf = self.install_type.get_compat_directory_safe()
            .context("Failed to get compatibility directory")?;
        let url = self.install_type.get_url(false);
        let release: Release = release_version(url.clone(), self.install_version.clone()).await?;
        let assets = get_asset_ids(self.install_type.get_extension(), release)?;
        let install_path = utils::get_download_directory_safe()?;
        // Create the clones of everything we need
        let mut install_loc = install_path.clone();
        let asset1 = assets[0].clone();
        let url_clone = url.clone();
        let multi_progress = std::sync::Arc::from(std::sync::Mutex::from(indicatif::MultiProgress::new()));
        let ref_1 = multi_progress.clone();
        let ref_2 = multi_progress.clone();
        let task1 = tokio::spawn( async move {
            install_loc.push(&asset1.name);
            handle_install(&install_loc, download_asset(url_clone, &asset1), &asset1.name, ref_1).await
        });
        let mut install_loc = install_path.clone();
        
        let task2 = tokio::spawn( async move {
            install_loc.push(&assets[1].name);
            handle_install(&install_loc, download_asset(url, &assets[1]), &assets[1].name, ref_2).await
        });
        
        let (tar_task_res, sha_task_res) = tokio::join!(task1, task2);
        
        let tar_path = tar_task_res.context("Failed to run download task for tar")??;
        let sha_path = sha_task_res.context("Failed to run download task for sha")??;

        // Newline isn't created between the progress bar. Until I've detatched printing from the
        // rest of the library, just add a newline
        term.write_fmt(format_args!("{}", prefix_style.apply_to("Checking hash ... "))).unwrap();
        if let Ok(is_match) = utils::check_sha(&tar_path, &sha_path) {
            if is_match {
                term.write_fmt(format_args!("{}", success_style.apply_to("Success\n"))).unwrap();
            } else {
                term.write_fmt(format_args!("{}", fail_style.apply_to("Fail\n"))).unwrap();
                return Err(anyhow::anyhow!("Hash mismatch error!"));
            }
        }
        term.write_fmt(format_args!("{}", prefix_style.apply_to("Decompressing ... "))).unwrap();
        match self.install_type {
            InstallTypeCmd::Wine => decompress::lzma(tar_path.clone(), compat_directory)?,
            InstallTypeCmd::Proton => decompress::gunzip(tar_path.clone(), compat_directory)?,
        }
        term.write_fmt(format_args!("{}", success_style.apply_to("Success\n"))).unwrap();
        term.write_line(format!("{}", prefix_style.apply_to("Removing artifacts")).as_str()).unwrap();
        utils::remove_download_pair(&[tar_path, sha_path]);
        Ok(())
    }


}
async fn handle_install(path: &std::path::PathBuf, fut: impl std::future::Future<Output = Result<Response, reqwest::Error>>, name: &String,
                        multi: std::sync::Arc<std::sync::Mutex<indicatif::MultiProgress>>) -> anyhow::Result<std::path::PathBuf> {
    let mut res = fut.await.context("Failed to get response")?;
    let mut total_install: u64 = 0;
    let length = if let Some(x) = res.content_length() {
        x
    } else {
        0
    };
    let length_hr = indicatif::HumanBytes(length);

    let style = indicatif::ProgressStyle::with_template("{prefix:.bold} {bar} {msg:.dim}").context("Failed to create progress bar style")?;
    let pb = multi.lock().unwrap().add(indicatif::ProgressBar::new(length));
    pb.set_prefix(format!("Downloading: {}", name));
    pb.set_style(style);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .context(format!("Failed to open file: {:?}", path))?;
    while let Ok(r) = res.chunk().await {
        if let Some(chunk) = r {
           if file.write_all(&chunk).is_ok() {
               let chunk_size = chunk.len() as u64;
               total_install += chunk_size;
               pb.inc(chunk_size);
               pb.set_message(format!("{}/{}", indicatif::HumanBytes(total_install), length_hr));
           }
        } else {
            break;
        }
    }
    pb.finish();
    Ok(path.to_path_buf())
}
