use anyhow;
use tokio;
use crate::github;
use crate::constants;

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
    compat_folder.push(".steam/root/compatibilitytools.d/");
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
