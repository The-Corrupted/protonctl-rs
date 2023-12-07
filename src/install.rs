use clap::Args;
use std::io::Write;
use reqwest::Response;
use anyhow::Context;
use console::{Term, Style};
use crate::cmd::InstallTypeCmd;
use protonctllib::{utils, decompress, github::api::{release_version, get_asset_ids, download_asset, Release}};

#[derive(Args, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Install {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(required = true)]
    install_version: String,
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
        let (asset1d, asset2d) = (assets[0].clone(), assets[1].clone());
        let (url_clone1d, url_clone2d) = (url.clone(), url.clone());
        let d_task1 = tokio::spawn(async move {download_asset(&url_clone1d, &asset1d).await});
        let d_task2 = tokio::spawn(async move {download_asset(&url_clone2d, &asset2d).await});
        let (mut res1, mut res2) = match tokio::join!(d_task1, d_task2) {
            (Ok(res1), Ok(res2)) => {
                (res1?, res2?)
            },
            _ => {return Err(anyhow::anyhow!("Failed to start downloads"));}
        };

        let max_size = if let Some(c1) = res1.content_length() {
            if let Some(c2) = res2.content_length() {
                c1 + c2
            } else {
                c1
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get content length"));
        };
        
        // Setup progress bar then put it into an Arc so it can be shared by the install tasks
        let pb = indicatif::ProgressBar::new(max_size);
        pb.set_style(indicatif::ProgressStyle::with_template("{prefix:.bold} {bar} {msg:.dim}")?);
        pb.set_prefix("Download:");
        let pb_wrapped = std::sync::Arc::from(std::sync::Mutex::from(pb));
        let pb_ref1 = pb_wrapped.clone();
        let (asset1, asset2) = (assets[0].clone(), assets[1].clone());
        let s1 = max_size;
        let task1 = tokio::spawn( async move {
            install_loc.push(&asset1.name);
            handle_install(&install_loc, &mut res1, pb_ref1, s1).await
        });
        let mut install_loc = install_path.clone(); 
        let pb_ref2 = pb_wrapped.clone();
        let s2 = max_size;
        let task2 = tokio::spawn( async move {
            install_loc.push(&assets[1].name);
            handle_install(&install_loc, &mut res2, pb_ref2, s2).await
        });
        
        let (tar_task_res, sha_task_res) = tokio::join!(task1, task2);

        let tar_path = tar_task_res.context("Failed to run download task for tar")??;
        let sha_path = sha_task_res.context("Failed to run download task for sha")??;

        // Tasks should be complete at this point and if we've made it this far, there wasn't an
        // error. Set the progress bar to finish then drop it.
        pb_wrapped.lock().unwrap().finish();
        drop(pb_wrapped);

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
async fn handle_install(path: &std::path::PathBuf, response: &mut Response,
                        pb: std::sync::Arc<std::sync::Mutex<indicatif::ProgressBar>>,
                        max_size: u64) -> anyhow::Result<std::path::PathBuf> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .context(format!("Failed to open file: {:?}", path))?;
    let mut total_install = 0;
    let max_hr = indicatif::HumanBytes(max_size);
    while let Ok(r) = response.chunk().await {
        if let Some(chunk) = r {
           if file.write_all(&chunk).is_ok() {
               let chunk_size = chunk.len() as u64;
               total_install += chunk_size;
               pb.lock().unwrap().inc(chunk_size);
               pb.lock().unwrap().set_message(format!("{}/{}", indicatif::HumanBytes(total_install), max_hr));
           }
        } else {
            break;
        }
    }
    Ok(path.to_path_buf())
}
