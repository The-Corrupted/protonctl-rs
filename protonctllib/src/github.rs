// Structs/Helpers for the github releases api

pub mod api {
    use anyhow::Context;
    use reqwest;
    use serde::Deserialize;

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
        url: &String,
        per_page: Option<u8>,
        page: Option<u8>,
    ) -> anyhow::Result<Releases> {
        let pp: u8 = per_page.unwrap_or(10);
        let p: u8 = page.unwrap_or(1);

        let response = reqwest::Client::new()
            .get(url)
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

    pub async fn latest_release(url: &String) -> anyhow::Result<Release> {
        let response = reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .context("Failed to get latest release")?;
        response
            .json::<Release>()
            .await
            .context("Failed to deserialize response")
    }

    pub async fn release_version(mut url: String, version: &String) -> anyhow::Result<Release> {
        url.push_str("/tags/");
        url.push_str(version.as_str());
        let response = reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .context(format!("Failed to get release {}", version))?;
        response.json::<Release>()
            .await.context("Failed to get release")
    }

    pub fn get_asset_ids(
        extension: &String,
        release: &Release,
    ) -> anyhow::Result<[AssetId; 2]> {
        // Get the release assets and the release tar file
        let sha_postfix = ".sha512sum";
        let mut ids: [AssetId; 2] = [AssetId::default(), AssetId::default()];
        let assets = &release.assets;
        for asset in assets {
            if asset.name.ends_with(extension.as_str()) {
                let id = AssetId {
                    name: asset.name.clone(),
                    id: asset.id,
                    size: asset.size,
                };
                ids[0] = id;
                continue;
            }
            if asset.name.ends_with(sha_postfix) {
                let id = AssetId {
                    name: asset.name.clone(),
                    id: asset.id,
                    size: asset.size,
                };
                ids[1] = id;
            }
        }
        Ok(ids)
    }

    pub async fn download_asset(url: &String,
                                asset: &AssetId) -> Result<reqwest::Response, reqwest::Error> {
        let mut url = url.clone();
        url.push_str(format!("/assets/{}", asset.id).as_str());
        reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .header("Accept", "application/octet-stream")
            .send().await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn can_get_releases() -> anyhow::Result<()> {
        use crate::github::api::releases;
        use crate::install_type::InstallType;
        let install = InstallType::Proton;
        let result = releases(&install.get_url(false), Some(50), Some(1)).await?;
        assert_eq!(result.len(), 50);
        Ok(())
    }

    #[tokio::test]
    async fn can_get_latest_release() -> anyhow::Result<()> {
        use crate::github::api::latest_release;
        use crate::install_type::InstallType;
        
        let install = InstallType::Proton;
        let _result = latest_release(&install.get_url(true)).await?;
        Ok(())
    }

    #[tokio::test]
    async fn can_get_release_by_tag() -> anyhow::Result<()> {
        use crate::github::api::{release_version, Release};
        use crate::install_type::InstallType;
        let version: String = String::from("GE-Proton8-4");

        let install = InstallType::Proton;
        let release: Release = release_version(install.get_extension(), &String::from("GE-Proton8-4")).await?;
        assert_eq!(release.tag_name, version);
        Ok(())
    }

    #[tokio::test]
    async fn can_get_asset_ids() -> anyhow::Result<()> {
        use crate::github::api::{get_asset_ids, release_version, Release};
        use crate::install_type::InstallType;
        let install = InstallType::Proton;

        let release: Release = release_version(install.get_url(false), &String::from("GE-Proton8-4")).await?;
        let ids = get_asset_ids(&install.get_url(false), &release)?;
        assert_eq!(ids[0].name, String::from("GE-Proton8-4.tar.gz"));
        assert_eq!(ids[1].name, String::from("GE-Proton8-4.sha512sum"));
        Ok(())
    }
}
