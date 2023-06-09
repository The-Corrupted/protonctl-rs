use anyhow;
use clap::{Args};
use tokio;
use crate::github;
use crate::constants;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct List {
    #[arg(short = 'n', long, default_value_t = 10, required = false)]
    pub number: u8,
    #[arg(short = 'p', long, default_value_t = 1, required = false)]
    pub page: usize,
    #[arg(short = 'e', long, default_value_t = String::from("proton"), required = false)]
    pub emulator: String,
    #[arg(short = 'l', long, default_value_t = false, required = false)]
    pub local: bool
}

impl List {
    pub async fn run(&self) -> anyhow::Result<()> {
        if self.local {
            let versions = get_installed_versions().await?;
            for version in versions {
                let version = version.file_name();
                match version.to_str() {
                    Some(name) => {
                        println!("{}", name);
                    }
                    None => {
                        println!("Something went wrong converting {:?} to a string", version);
                    }
                }
            }
        } else {
            if let Some(releases) = get_releases_paged(self.number, self.page).await {
                for release in releases {
                    self.print_releases_formatted(release.get_version(), release.get_body(), release.get_release_url());
                }
            } else {
                return Err(anyhow::anyhow!("Failed to get releases"));
            }
        }
        Ok(())
    }

    fn print_releases_formatted(&self, version: String, body: String, url: String) {
        println!("Version: {}", version);
        println!("Download: {}", url);
        println!("{}", body);
        println!("--------------------\n");
    }
}

pub async fn get_releases_paged(mut number: u8, page: usize) -> Option<github::api::Releases> {
    if number > constants::MAX_PER_PAGE {
        number = constants::MAX_PER_PAGE
    }
    
    let releases_wrapped = github::api::releases(Some(number), Some(page)).await;
    let releases = match releases_wrapped {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    Some(releases)
}

pub async fn get_installed_versions() -> anyhow::Result<Vec<tokio::fs::DirEntry>> {
    let home: std::path::PathBuf = match constants::HOME_DIR.clone() {
        Some(h) => h,
        None => {
            return Err(anyhow::anyhow!("Failed to get home directory"));
        }
    };
    let mut compat_folder = home.to_owned();
    compat_folder.push(constants::STEAM_COMPAT_PATH.to_owned());
    let dir_entries_result = tokio::fs::read_dir(compat_folder).await;
    let mut entries: Vec<tokio::fs::DirEntry> = Vec::new();
    let mut dir_entries = match dir_entries_result {
        Ok(d) => d,
        Err(_e) => {
            return Err(anyhow::anyhow!("Failed to read compatibility directory. Does it exist?"));
        }
    };
    while let Ok(dir) = dir_entries.next_entry().await {
        match dir {
            Some(d) => {
                entries.push(d);
            }
            None => {
                break;
            }
        }
    };
    Ok(entries)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn can_get_local_dir() -> anyhow::Result<()> {
        use crate::list::get_installed_versions;
        let results = get_installed_versions().await?;
        if !results.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Directory was empty"))
        }
    }
}
