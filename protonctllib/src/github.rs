// Structs/Helpers for the github releases api

pub mod api {
    use std::io::Write;
    use crate::cmd::InstallType;
    use crate::{constants, os_helper};
    use anyhow::Context;
    use reqwest;
    use serde::Deserialize;
    use indicatif::{ProgressBar, ProgressStyle, HumanBytes};

    #[derive(Deserialize, Debug, Clone, Default)]
    pub struct AssetId {
        pub name: String,
        pub id: u64,
        pub size: u64,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Release {
        pub html_url: String,
        pub tag_name: String,
        pub assets: Vec<AssetId>,
        pub body: String,
    }


    pub type Releases = Vec<Release>;

    pub async fn releases(
        install_type: InstallType,
        per_page: Option<u8>,
        page: Option<u8>,
    ) -> anyhow::Result<Releases> {
        let pp: u8 = per_page.unwrap_or(10);
        let p: u8 = page.unwrap_or(1);

        let response = reqwest::Client::new()
            .get(get_url_path(install_type, false))
            .query(&[("per_page", pp), ("page", p)])
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .context("Failed to get releases")?;
        response
            .json::<Releases>()
            .await
            .context("Failed to deserialize response")
    }

    pub async fn latest_release(install_type: InstallType) -> anyhow::Result<Release> {
        let response = reqwest::Client::new()
            .get(get_url_path(install_type, true))
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .context("Failed to get latest release")?;
        response
            .json::<Release>()
            .await
            .context("Failed to deserialize response")
    }

    pub async fn release_version(install_type: InstallType, version: String) -> anyhow::Result<Release> {
        let mut release_url = get_url_path(install_type, false);
        release_url.push_str("/tags/");
        release_url.push_str(version.as_str());
        let response = reqwest::Client::new()
            .get(release_url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .context(format!("Failed to get release {}", version))?;
        response.json::<Release>()
            .await.context("Failed to get release")
    }

    pub fn get_asset_ids(
        install_type: InstallType,
        release: Release,
    ) -> anyhow::Result<[AssetId; 2]> {
        // Get the release assets and the release tar file
        let compression_postfix = match install_type {
            InstallType::Wine => ".tar.xz",
            InstallType::Proton => ".tar.gz",
        };
        let sha_postfix = ".sha512sum";
        let mut ids: [AssetId; 2] = [AssetId::default(), AssetId::default()];
        let assets = release.assets;
        for asset in assets {
            if asset.name.ends_with(compression_postfix) {
                let id = AssetId {
                    name: asset.name,
                    id: asset.id,
                    size: asset.size,
                };
                ids[0] = id;
                continue;
            }
            if asset.name.ends_with(sha_postfix) {
                let id = AssetId {
                    name: asset.name,
                    id: asset.id,
                    size: asset.size,
                };
                ids[1] = id;
            }
        }
        Ok(ids)
    }

    pub async fn download_assets(
        install_type: InstallType,
        asset_ids: [AssetId; 2],
    ) -> anyhow::Result<[std::path::PathBuf; 2]> {
        // Start downloading files
        let mut downloaded_files: [std::path::PathBuf; 2] =
            [std::path::PathBuf::new(), std::path::PathBuf::new()];
        // Get progress bar stuff setup
        let total_size = asset_ids[0].size + asset_ids[1].size;

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::with_template("{prefix:.bold} {bar:40} {msg:.dim}")
            .unwrap());
        pb.set_prefix("Download:");
        let p_human = HumanBytes(total_size).to_string();
        let mut bytes_read = 0;

        for x in 0..asset_ids.len() {
            let asset = asset_ids[x].clone();
            let mut asset_path = get_url_path(install_type, false);
            asset_path.push_str(format!("/assets/{}", asset.id).as_str());
            let mut response = reqwest::Client::new()
                .get(asset_path)
                .header("user-agent", "protonctl-rs")
                .header("Accept", "application/octet-stream")
                .send()
                .await
                .context("Failed to get asset ID")?;
            if response.status().is_success() {
                // We got what we wanted. Stream the body to file
                let mut path = os_helper::get_download_directory_safe()?;
                path.push(&asset.name);
                let mut file_handle = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&path)
                    .context("Failed to open file for writing")?;
               while let Ok(chunk) = response.chunk().await {
                   if let Some(bytes) = chunk {
                       let read = bytes.len() as u64; 
                       bytes_read += read;
                       if let Err(_e) = file_handle.write_all(&bytes) {
                           println!("Failed to write chunk");
                           return Err(anyhow::anyhow!("Chunk failed to write to file"));
                       }
                       pb.inc(read);
                       pb.set_message(format!("{}/{}", HumanBytes(bytes_read), p_human));
                   } else {
                       break;
                   }
               }
               downloaded_files[x] = path;
           }
        }
        pb.finish();
        Ok(downloaded_files)
    }

    // Get the proper url based on the selected install type
    fn get_url_path(install_type: InstallType, is_latest: bool) -> String {
        let mut url = if is_latest {
            constants::LATEST_PATH.to_owned()
        } else {
            constants::RELEASES_PATH.to_owned()
        };
        url = url.replacen("{}", constants::PROJECT_OWNER, 1);
        match install_type {
            InstallType::Proton => url.replacen("{}", constants::PROTON_PROJECT_NAME, 1),
            InstallType::Wine => url.replacen("{}", constants::WINE_PROJECT_NAME, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn can_get_releases() -> anyhow::Result<()> {
        use crate::cmd::InstallType;
        use crate::github::api::releases;
        let result = releases(InstallType::Proton, Some(50), Some(1)).await?;
        assert_eq!(result.len(), 50);
        Ok(())
    }

    #[tokio::test]
    async fn can_get_latest_release() -> anyhow::Result<()> {
        use crate::cmd::InstallType;
        use crate::github::api::latest_release;
        let _result = latest_release(InstallType::Proton).await?;
        Ok(())
    }

    #[tokio::test]
    async fn can_get_release_by_tag() -> anyhow::Result<()> {
        use crate::cmd::InstallType;
        use crate::github::api::{release_version, Release};
        let version: String = String::from("GE-Proton8-4");
        let release: Release = release_version(InstallType::Proton, String::from("GE-Proton8-4")).await?;
        assert_eq!(release.tag_name, version);
        Ok(())
    }

    #[tokio::test]
    async fn can_get_asset_ids() -> anyhow::Result<()> {
        use crate::cmd::InstallType;
        use crate::github::api::{get_asset_ids, release_version, Release};

        let release: Release = release_version(InstallType::Proton, String::from("GE-Proton8-4")).await?;
        let ids = get_asset_ids(InstallType::Proton, release)?;
        assert_eq!(ids[0].name, String::from("GE-Proton8-4.tar.gz"));
        assert_eq!(ids[1].name, String::from("GE-Proton8-4.sha512sum"));
        Ok(())
    }
}
