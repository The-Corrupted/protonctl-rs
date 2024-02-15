use crate::constants::MAX_PER_PAGE;
use crate::github;
use anyhow;
use dirs::home_dir;

pub async fn get_releases_paged(
    url: String,
    mut number: u8,
    page: u8,
) -> Option<github::api::Releases> {
    if number > MAX_PER_PAGE {
        number = MAX_PER_PAGE
    }

    let releases_wrapped = github::api::releases(&url, Some(number), Some(page));
    let releases = match releases_wrapped.await {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    Some(releases)
}

pub fn get_installed_versions(path: &std::path::PathBuf) -> anyhow::Result<Vec<std::fs::DirEntry>> {
    let mut home: std::path::PathBuf =
        home_dir().ok_or(anyhow::anyhow!("Couldn't get users home directory"))?;
    home.push(path);
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
        use crate::install_type::InstallType;
        use crate::version_info::get_installed_versions;

        let install = InstallType::Proton;
        let results = get_installed_versions(&install.get_compat_directory_safe().unwrap())?;
        if !results.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Directory was empty"))
        }
    }
}
