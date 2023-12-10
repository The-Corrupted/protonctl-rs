use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use std::{io::Write, sync::{Arc, Mutex}};
use reqwest::Response;
use anyhow::Context;
use console::{Term, Style};
use crate::cmd::InstallTypeCmd;
use protonctllib::{utils, decompress, github::api::{release_version, get_asset_ids, download_asset, Release}};

// We would like install_type to be position AND skippable. You're currently unable to skip it
// so until I figure out why this is, it's technically mandatory.
#[derive(Args, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Install {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(required = true)]
    install_version: String,
}

// Meant to be used in an Arc<Mutex>. Contains the preconfigured progress bar,
// and the u64 ( which should be the total download size ). This is mostly
// being used to reduce the number of arguments install_handler takes as
// well as the amount of manual cloning we do in run
#[derive(Debug, Clone)]
pub(crate) struct SharedProgressBar {
    max: u64,
    pb: ProgressBar,
}

impl SharedProgressBar {
    pub fn new(max: u64) -> anyhow::Result<Self> {
        let style = ProgressStyle::with_template("{prefix:.bold} {bar} {msg:.dim}")
            .context("Failed to set style")?;
        let pb = ProgressBar::new(max);
        pb.set_style(style);
        pb.set_prefix("Downloading:");
        Ok(Self {
            max,
            pb
        })
    }
}


// Struct containing all the styles we use in the run function.
// It may be worth allowing the user to customize in the future.
pub(crate) struct Styles {
    success_style: Style,
    fail_style: Style,
    prefix_style: Style
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
        let compat_directory: std::path::PathBuf = self.install_type.get_compat_directory_safe()
            .context("Failed to get compatibility directory")?;
        let url = self.install_type.get_url(false);
        let release: Release = release_version(url.clone(), &self.install_version).await?;
        let assets = get_asset_ids(&self.install_type.get_extension(), &release)?;
        let install_path = utils::get_download_directory_safe()?;
        // Create the clones of everything we need
        let mut install_loc = install_path.clone();
        let (asset1d, asset1i, asset2d, asset2i) = (assets[0].clone(), assets[0].clone(), assets[1].clone(), assets[1].clone());
        let (url_clone1, url_clone2) = (url.clone(), url.clone());
        let d_task1 = tokio::spawn(async move {download_asset(&url_clone1, &asset1d).await});
        let d_task2 = tokio::spawn(async move {download_asset(&url_clone2, &asset2d).await});
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
        let shared_bar = Arc::from(Mutex::from(SharedProgressBar::new(max_size)?));
        let (shared_bar1, shared_bar2) = (shared_bar.clone(), shared_bar.clone());
        let task1 = tokio::spawn( async move {
            install_loc.push(&asset1i.name);
            handle_install(&install_loc, &mut res1, shared_bar1).await
        });

        let mut install_loc = install_path.clone(); 
        let task2 = tokio::spawn( async move {
            install_loc.push(&asset2i.name);
            handle_install(&install_loc, &mut res2, shared_bar2).await
        });
        
        let (tar_task_res, sha_task_res) = tokio::join!(task1, task2);

        // Having multiple ?'s isn't ideal but join returns a result and so does handle_install
        // so we need to unwrap twice to get to the path buffer
        let tar_path = tar_task_res.context("Failed to run download task for tar")??;
        let sha_path = sha_task_res.context("Failed to run download task for sha")??;

        // Tasks should be complete at this point and if we've made it this far, there wasn't an
        // error. Set the progress bar to finish then drop it.
        shared_bar.lock().unwrap().pb.finish();
        drop(shared_bar);
        
        // Check and make sure the file hash matches the SHA512 file. If it doesn't we shouldn't
        // unpack it. Indicate a failure has occured and 
        term.write_fmt(format_args!("{}", styles.prefix_style.apply_to("Checking hash ... "))).unwrap();
        match utils::check_sha(&tar_path, &sha_path) {
            Ok(is_match) => {
                if is_match {
                    term.write_fmt(format_args!("{}", styles.success_style.apply_to("Success\n"))).unwrap();
                } else {
                    term.write_fmt(format_args!("{}", styles.fail_style.apply_to("Fail\n"))).unwrap();
                    return Err(anyhow::anyhow!("Hash mismatch error!"));
                }
            }
            Err(e) => {return Err(e);}
        } 
        
        // Decompress the file based on the install_type. We may change this later...
        term.write_fmt(format_args!("{}", styles.prefix_style.apply_to("Decompressing ... "))).unwrap();
        match self.install_type {
            InstallTypeCmd::Wine => decompress::lzma(tar_path.clone(), compat_directory)?,
            InstallTypeCmd::Proton => decompress::gunzip(tar_path.clone(), compat_directory)?,
        }
        // Nothing has failed and we've reached the end. Remove downloaded files and exit
        term.write_fmt(format_args!("{}", styles.success_style.apply_to("Success\n"))).unwrap();
        term.write_line(format!("{}", styles.prefix_style.apply_to("Removing artifacts")).as_str()).unwrap();
        utils::remove_download_pair(&[tar_path, sha_path]);
        Ok(())
    }


}
async fn handle_install(path: &std::path::PathBuf, response: &mut Response,
                        shared_bar: Arc<Mutex<SharedProgressBar>>) -> anyhow::Result<std::path::PathBuf> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .context(format!("Failed to open file: {:?}", path))?;
    let mut total_install = 0;
    let max_hr = indicatif::HumanBytes(shared_bar.lock().unwrap().max);
    while let Ok(r) = response.chunk().await {
        if let Some(chunk) = r {
           if file.write_all(&chunk).is_ok() {
               let chunk_size = chunk.len() as u64;
               total_install += chunk_size;
               let sb = shared_bar.lock().unwrap();
               sb.pb.inc(chunk_size);
               sb.pb.set_message(format!("{}/{}", indicatif::HumanBytes(total_install), max_hr));
           }
        } else {
            break;
        }
    }
    Ok(path.to_path_buf())
}
