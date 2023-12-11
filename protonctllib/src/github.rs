// Structs/Helpers for the github releases api

pub mod api {
    use reqwest;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Clone, Default)]
    pub struct AssetId {
        pub name: String,
        pub id: u64,
        pub size: u64,
    }

    impl AssetId {
        pub fn is_empty(&self) -> bool {
            self.name.is_empty()
        }
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
        url: &str,
        per_page: Option<u8>,
        page: Option<u8>,
    ) -> Result<Releases, reqwest::Error> {
        let pp: u8 = per_page.unwrap_or(10);
        let p: u8 = page.unwrap_or(1);

        reqwest::Client::new()
            .get(url)
            .query(&[("per_page", pp), ("page", p)])
            .header("user-agent", "protonctl-rs")
            .send()
            .await?
            .json::<Releases>()
            .await
    }

    pub async fn latest_release(url: &str) -> Result<Release, reqwest::Error> {
        reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await?
            .json::<Release>()
            .await
    }

    pub async fn release_version(url: &str, version: &str) -> Result<Release, reqwest::Error> {
        let mut url = url.to_owned();
        url.push_str("/tags/");
        url.push_str(version);
        reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await?
            .json::<Release>()
            .await
    }

    pub fn get_asset_ids(release: &Release) -> (AssetId, AssetId) {
        // Get the release assets and the release tar file
        let extensions = [".tar.gz", ".tar.xz"];
        let sha_postfix = ".sha512sum";
        let mut tar_asset: AssetId = AssetId::default();
        let mut sha_asset: AssetId = AssetId::default();
        let assets = &release.assets;
        for asset in assets {
            if asset.name.ends_with(&extensions[0]) || asset.name.ends_with(&extensions[1]) {
                let id = AssetId {
                    name: asset.name.clone(),
                    id: asset.id,
                    size: asset.size,
                };
                tar_asset = id;
                continue;
            }
            if asset.name.ends_with(sha_postfix) {
                let id = AssetId {
                    name: asset.name.clone(),
                    id: asset.id,
                    size: asset.size,
                };
                sha_asset = id;
            }

            if !tar_asset.is_empty() && !sha_asset.is_empty() {
                break;
            }
        }
        (tar_asset, sha_asset)
    }

    pub async fn download_asset(
        mut url: String,
        asset: &AssetId,
    ) -> Result<reqwest::Response, reqwest::Error> {
        url.push_str(format!("/assets/{}", asset.id).as_str());
        reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .header("Accept", "application/octet-stream")
            .send()
            .await
    }

    pub async fn download_asset_to_memory(
        mut url: String,
        asset: &AssetId,
    ) -> Result<String, reqwest::Error> {
        url.push_str(format!("/assets/{}", asset.id).as_str());
        reqwest::Client::new()
            .get(url)
            .header("user-agent", "protonctl-rs")
            .header("Accept", "application/octet-stream")
            .send()
            .await?
            .text()
            .await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn can_get_releases() -> Result<(), reqwest::Error> {
        use crate::github::api::releases;
        use crate::install_type::InstallType;
        let install = InstallType::Proton;
        let result = releases(&install.get_url(false), Some(50), Some(1)).await?;
        assert_eq!(result.len(), 50);
        Ok(())
    }

    #[tokio::test]
    async fn can_get_latest_release() -> Result<(), reqwest::Error> {
        use crate::github::api::latest_release;
        use crate::install_type::InstallType;

        let install = InstallType::Proton;
        let _result = latest_release(&install.get_url(true)).await?;
        Ok(())
    }

    #[tokio::test]
    async fn can_get_release_by_tag() -> Result<(), reqwest::Error> {
        use crate::github::api::{release_version, Release};
        use crate::install_type::InstallType;
        let version: String = String::from("GE-Proton8-4");

        let install = InstallType::Proton;
        let release: Release =
            release_version(&install.get_url(false), &String::from("GE-Proton8-4")).await?;
        assert_eq!(release.tag_name, version);
        Ok(())
    }

    #[tokio::test]
    async fn can_get_asset_ids() -> Result<(), reqwest::Error> {
        use crate::github::api::{get_asset_ids, release_version, Release};
        use crate::install_type::InstallType;
        let install = InstallType::Proton;

        let release: Release =
            release_version(&install.get_url(false), &String::from("GE-Proton8-4")).await?;
        let (tar_asset, sha_asset) = get_asset_ids(&release);
        assert_eq!(tar_asset.name, String::from("GE-Proton8-4.tar.gz"));
        assert_eq!(sha_asset.name, String::from("GE-Proton8-4.sha512sum"));
        Ok(())
    }
}
