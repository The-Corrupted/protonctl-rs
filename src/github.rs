// Structs/Helpers for the github releases api
// Improve errors. We currently just return a String if there is an error
// to avoid the complications of potentially returning multiple different
// errors from the same function. This is a 'dumb' way to do error handling
// and we should do it the 'right' way

pub mod api {
    use std::collections::HashMap;
    use serde::{Deserialize};
    use reqwest;
    use anyhow;
    use crate::constants;

    
    #[derive(Debug, Deserialize, Clone)]
    pub struct User {
        login: String,
        id: usize,
        node_id: String,
        avatar_url: String,
        gravatar_id: String,
        url: String,
        html_url: String,
        followers_url: String,
        following_url: String,
        gists_url: String,
        starred_url: String,
        subscriptions_url: String,
        organizations_url: String,
        repos_url: String,
        events_url: String,
        received_events_url: String,
        #[serde(alias = "type")]
        user_type: String,
        site_admin: bool,
    }
    
    #[derive(Debug, Deserialize, Clone)]
    pub struct Assets {
        url: String,
        id: usize,
        node_id: String,
        name: String,
        label: Option<String>,
        uploader: User,
        content_type: String,
        state: String,
        size: usize,
        download_count: usize,
        created_at: String,
        updated_at: String,
        browser_download_url: String,

    }
    
    #[derive(Debug, Deserialize, Clone)]
    pub struct Reactions {
        url: String,
        total_count: usize,
        #[serde(alias = "+1")]
        plus_one: usize,
        #[serde(alias = "-1")]
        minus_one: usize,
        laugh: usize,
        hooray: usize,
        confused: usize,
        heart: usize,
        rocket: usize,
        eyes: usize,
    }
    
    #[derive(Debug, Deserialize, Clone)]
    pub struct Release {
        url: String,
        assets_url: String,
        upload_url: String,
        html_url: String,
        id: usize,
        author: User,
        node_id: String,
        tag_name: String,
        target_commitish: String,
        name: String,
        draft: bool,
        prerelease: bool,
        created_at: String,
        published_at: String,
        assets: Vec<Assets>,
        tarball_url: String,
        zipball_url: String,
        body: String,
        reactions: Reactions
    }
    
    #[derive(Debug, Clone, Default)]
    pub struct AssetId {
        pub name: String,
        pub id: usize,
    }

    impl Release {
        pub fn get_version(self: &Self) -> String {
            self.tag_name.clone()
        }

       pub fn get_assets(self: &Self) -> Vec<Assets> {
           self.assets.clone()
       }

       pub fn get_release_url(self: &Self) -> String {
           self.html_url.clone()
       }
            
        pub fn get_body(self: &Self) -> String {
            self.body.clone()
        }
    }
    

    pub type Releases = Vec<Release>;

    pub async fn releases(per_page: Option<u8>, page: Option<usize>) -> anyhow::Result<Releases> {
        let pp: String = if let Some(number) = per_page {
            number.to_string()
        } else { String::from("10") };
        let p: String = if let Some(number) = page {
            number.to_string()
        } else { String::from("1") };
        let response = reqwest::Client::new()
            .get(constants::PROTON_GE_RELEASE_PATH)
            .query(&[("per_page", pp.to_string()),
            ("page", p.to_string())])
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .or_else(|e|
                convert_reqwest_error("Failed to get releases", e))?;
        response.json::<Releases>().await.or_else(|e| 
            convert_reqwest_error("Failed to deserialize response",e))
    }

    pub async fn latest_release() -> anyhow::Result<Release> {
        let response = reqwest::Client::new()
            .get(constants::PROTON_GE_LATEST_PATH)
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .or_else(|e|
                convert_reqwest_error("Failed to get latest release", e))?;
        response.json::<Release>().await.or_else(|e|
            convert_reqwest_error("Failed to deserialize response", e))
    }

    pub async fn release_version(version: String) -> anyhow::Result<Release> {
        let mut release_url = constants::PROTON_GE_RELEASE_PATH.to_string();
        release_url.push_str("/tags/");
        release_url.push_str(version.as_str());
        let response = reqwest::Client::new()
            .get(release_url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await
            .or_else(|e| 
                convert_reqwest_error(format!("Failed to get release {}", version), e))?;
        response.json::<Release>().await.or_else(|e| 
            convert_reqwest_error("Failed to get release", e))
    }

    pub async fn get_asset_ids(release: Release) -> anyhow::Result<[AssetId;2]> {
        // Get the release assets and the release tar file
        let version: String = release.get_version();
        let tar_ball: String = format!("{}.tar.gz", version);
        let sha512sum: String = format!("{}.sha512sum", version);
        let mut ids: [AssetId;2] = [AssetId::default(), AssetId::default()];
        let assets = release.get_assets();
        for asset in assets {
            if asset.name == tar_ball {
                let id = AssetId {
                    name: asset.name,
                    id: asset.id
                };
                ids[0] = id;
                continue;
            }
            if asset.name == sha512sum {
                let id = AssetId {
                    name: asset.name,
                    id: asset.id
                };
                ids[1] = id;
            }
        }
        Ok(ids)
    }

    pub async fn download_assets(asset_ids: [AssetId;2]) -> anyhow::Result<()> {
        Ok(())
    }

    /* Reqwest has its own error type that seems to be incompatible with anyhow.
     * For the sake of not returning loads of different error types, just convert
     * the reqwest error to an anyhow error. I'm probably doing this poorly
     * and should look into better ways of handling errors
    */
    fn convert_reqwest_error<S, T>(message: S, e: reqwest::Error) -> Result<T, anyhow::Error>
    where S: ToString + std::fmt::Display,
    {
        Err(anyhow::anyhow!("{}: {:?}", message, e))
    }
}

    
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn can_get_releases() -> anyhow::Result<()> {
        use crate::github::api::releases;
        let result = releases(Some(50), Some(1)).await?;
        assert_eq!(result.len(), 50);
        Ok(())
    }
    
    #[tokio::test]
    async fn can_get_latest_release() -> anyhow::Result<()> {
        use crate::github::api::latest_release;
        let result = latest_release().await?;
        Ok(())
    }

    #[tokio::test]
    async fn can_get_release_by_tag() -> anyhow::Result<()> {
        use crate::github::api::{Release, release_version};
        let version: String = String::from("GE-Proton8-4");
        let release: Release = release_version(String::from("GE-Proton8-4")).await?;
        assert_eq!(release.get_version(), String::from("GE-Proton8-4"));
        Ok(())
    }

    #[tokio::test]
    async fn can_get_asset_ids() -> anyhow::Result<()> {
        use crate::github::api::{Release, AssetId, release_version, get_asset_ids};
        let release: Release = release_version(String::from("GE-Proton8-4")).await?;
        let ids = get_asset_ids(release).await?;
        assert_eq!(ids[0].name, String::from("GE-Proton8-4.tar.gz"));
        assert_eq!(ids[1].name, String::from("GE-Proton8-4.sha512sum"));
        Ok(())
    }
}
