use crate::cmd::{Run, InstallType};
use crate::constants;
use crate::github;
use anyhow;
use clap::Args;

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct List {
    #[arg(short = 'n', long, default_value_t = 10, required = false)]
    pub number: u8,
    #[arg(short = 'p', long, default_value_t = 1, required = false)]
    pub page: u8,
    #[arg(short = 'l', long, default_value_t = false, required = false)]
    pub local: bool,
}

impl Run for List {
    fn run(&self, install_type: InstallType) -> anyhow::Result<()> {
        if self.local {
            let versions = get_installed_versions(install_type)?;
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
        } else if let Some(releases) = get_releases_paged(install_type, self.number, self.page) {
            for release in releases {
                print_releases_formatted(
                    release.tag_name,
                    release.body,
                    release.html_url,
                );
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get releases"));
        }
        Ok(())
    }
}

fn print_releases_formatted(version: String, body: String, url: String) {
    println!("Version: {}", version);
    println!("Download: {}", url);
    println!("{}", body);
    println!("--------------------\n");
}

pub fn get_releases_paged(install_type: InstallType, mut number: u8, page: u8) -> Option<github::api::Releases> {
    if number > constants::MAX_PER_PAGE {
        number = constants::MAX_PER_PAGE
    }

    let releases_wrapped = github::api::releases(install_type, Some(number), Some(page));
    let releases = match releases_wrapped {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    Some(releases)
}

pub fn get_installed_versions(install_type: InstallType) -> anyhow::Result<Vec<std::fs::DirEntry>> {
    let mut home: std::path::PathBuf = match constants::HOME_DIR.clone() {
        Some(h) => h.to_owned(),
        None => {
            return Err(anyhow::anyhow!("Failed to get home directory"));
        }
    };
    let compat_path = match install_type {
        InstallType::Wine => constants::LUTRIS_RUNNERS_PATH.to_owned(),
        InstallType::Proton => constants::STEAM_COMPAT_PATH.to_owned(),
    };
    home.push(compat_path);
    let dir_entries_result = std::fs::read_dir(home);
    let mut entries: Vec<std::fs::DirEntry> = Vec::new();
    let dir_entries = match dir_entries_result {
        Ok(d) => d,
        Err(_e) => {
            return Err(anyhow::anyhow!(
                "Failed to read compatibility directory. Does it exist?"
            ));
        }
    };
    for dir in dir_entries {
        match dir {
            Ok(d) => {
                entries.push(d);
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_get_local_dir() -> anyhow::Result<()> {
        use crate::list::get_installed_versions;
        use crate::cmd::InstallType;
        let results = get_installed_versions(InstallType::Proton)?;
        if !results.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Directory was empty"))
        }
    }
}
