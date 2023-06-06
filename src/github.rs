// Structs/Helpers for the github releases api
// Improve errors. We currently just return a String if there is an error
// to avoid the complications of potentially returning multiple different
// errors from the same function. This is a 'dumb' way to do error handling
// and we should do it the 'right' way

pub mod api {
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

    pub async fn releases(per_page: Option<u8>, page: Option<usize>) -> reqwest::Result<Releases> {
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
            .await?;
        response.json::<Releases>().await
    }

    pub async fn latest_release() -> reqwest::Result<Release> {
        let response = reqwest::Client::new()
            .get(constants::PROTON_GE_LATEST_PATH)
            .header("user-agent", "protonctl-rs")
            .send()
            .await?;
        response.json::<Release>().await
    }

    pub async fn release_version(version: String) -> reqwest::Result<Release> {
        let mut release_url = constants::PROTON_GE_RELEASE_PATH.to_string();
        release_url.push_str("/tags/");
        release_url.push_str(version.as_str());
        let response = reqwest::Client::new()
            .get(release_url)
            .header("user-agent", "protonctl-rs")
            .send()
            .await?;
        response.json::<Release>().await
    }

    pub async fn get_release_asset_ids(release: Release) -> reqwest::Result<[usize;2]> {
        // Get the release assets and the release tar file
        let mut assets_url = constants::PROTON_GE_RELEASE_PATH.to_string();
        let version: String = release.get_version();
        let tar_ball: String = format!("{}.tar.gz", version);
        let sha512sum: String = format!("{}.sha512sum", version);
        assets_url.push_str("/assets/");
        let assets = release.get_assets();
        let mut asset_ids: [usize;2] = [0;2];
        for asset in assets {
            if asset.name == tar_ball {
                asset_ids[0] = asset.id;
            }
            if asset.name == sha512sum {
                asset_ids[1] = asset.id;
            }
        }
        Ok(asset_ids)
    }

    pub async fn download_release_assets(asset_ids: [usize;2]) -> reqwest::Result<()> {


        Ok(())
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
        use crate::github::api::{Release, release_version, get_release_asset_ids};
        let release: Release = release_version(String::from("GE-Proton8-4")).await?;
        let asset_ids: [usize;2] = get_release_asset_ids(release).await?;
        assert_eq!(asset_ids[0], 111405985);
        assert_eq!(asset_ids[1], 111405984);
        Ok(())
    }
}
