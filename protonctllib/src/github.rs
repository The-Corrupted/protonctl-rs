// Structs/Helpers for the github releases api
// Improve errors. We currently just return a String if there is an error
// to avoid the complications of potentially returning multiple different
// errors from the same function. This is a 'dumb' way to do error handling
// and we should do it the 'right' way

pub mod api {
    use crate::{constants, os_helper};
    use crate::cmd::InstallType;
    use anyhow;
    use reqwest::blocking;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Clone)]
    pub struct Assets {
        pub id: usize,
        pub name: String,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct Release {
        pub html_url: String,
        pub tag_name: String,
        pub assets: Vec<Assets>,
        pub body: String,
    }

    #[derive(Debug, Clone, Default)]
    pub struct AssetId {
        pub name: String,
        pub id: usize,
    }

    pub type Releases = Vec<Release>;

    pub fn releases(install_type: InstallType, per_page: Option<u8>, page: Option<u8>) -> anyhow::Result<Releases> {
        let pp: u8 = if let Some(number) = per_page {
            number
        } else {
            10
        };
        let p: u8 = if let Some(number) = page {
            number
        } else {
            1
        };

        let response = blocking::Client::new()
            .get(get_url_path(install_type, false))
            .query(&[("per_page", pp), ("page", p)])
            .header("user-agent", "protonctl-rs")
            .send()
            .or_else(|e| convert_reqwest_error("Failed to get releases", e))?;
        response
            .json::<Releases>()
            .or_else(|e| convert_reqwest_error("Failed to deserialize response", e))
    }

    pub fn latest_release(install_type: InstallType) -> anyhow::Result<Release> {
        let response = blocking::Client::new()
            .get(get_url_path(install_type, true))
            .header("user-agent", "protonctl-rs")
            .send()
            .or_else(|e| convert_reqwest_error("Failed to get latest release", e))?;
        response
            .json::<Release>()
            .or_else(|e| convert_reqwest_error("Failed to deserialize response", e))
    }

    pub fn release_version(install_type: InstallType, version: String) -> anyhow::Result<Release> {
        let mut release_url = get_url_path(install_type, false);
        release_url.push_str("/tags/");
        release_url.push_str(version.as_str());
        let response = blocking::Client::new()
            .get(release_url)
            .header("user-agent", "protonctl-rs")
            .send()
            .or_else(|e| convert_reqwest_error(format!("Failed to get release {}", version), e))?;
        response
            .json::<Release>()
            .or_else(|e| convert_reqwest_error("Failed to get release", e))
    }

    pub fn get_asset_ids(release: Release) -> anyhow::Result<[AssetId; 2]> {
        // Get the release assets and the release tar file
        let version: String = release.tag_name;
        let tar_ball: String = format!("{}.tar.gz", version);
        let sha512sum: String = format!("{}.sha512sum", version);
        let mut ids: [AssetId; 2] = [AssetId::default(), AssetId::default()];
        let assets = release.assets;
        for asset in assets {
            if asset.name == tar_ball {
                let id = AssetId {
                    name: asset.name,
                    id: asset.id,
                };
                ids[0] = id;
                continue;
            }
            if asset.name == sha512sum {
                let id = AssetId {
                    name: asset.name,
                    id: asset.id,
                };
                ids[1] = id;
            }
        }
        Ok(ids)
    }

    pub fn download_assets(install_type: InstallType, asset_ids: [AssetId; 2]) -> anyhow::Result<[std::path::PathBuf; 2]> {
        println!("Downloading tar and sha files");
        let mut downloaded_files: [std::path::PathBuf; 2] = [std::path::PathBuf::new(), std::path::PathBuf::new()];
        for x in 0..asset_ids.len() {
            let asset = asset_ids[x].clone();
            let mut asset_path = get_url_path(install_type, false);
            asset_path.push_str(format!("/assets/{}", asset.id).as_str());
            let mut response = blocking::Client::new()
                .get(asset_path)
                .header("user-agent", "protonctl-rs")
                .header("Accept", "application/octet-stream")
                .send()
                .or_else(|e| convert_reqwest_error("Failed to get asset", e))?;
            if response.status().is_success() {
                // We got what we wanted. Stream the body to file
                let mut path = os_helper::get_install_directory_safe()?;
                path.push(&asset.name);
                let mut file_handle = match std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&path)
                {
                    Ok(f) => f,
                    Err(_e) => {
                        return Err(anyhow::anyhow!("Failed to open file: {:?}", path));
                    }
                };
                match response.copy_to(&mut file_handle) {
                    Ok(e) => {
                        // We successfully got the file. Print success status and add it to the
                        // installed files array. We will need this for decompression and sha512sum 
                        // checks
                        let (units, prefix) = bytes_conversion(e);
                        println!("File {} written. Wrote {} {}", asset.name, units, prefix);
                        downloaded_files[x] = path;
                    }
                    Err(e) => return convert_reqwest_error("Failed to write to file", e),
                }
            }
        }
        Ok(downloaded_files)
    }
    
    // Get the proper url based on the selected install type
    fn get_url_path(install_type: InstallType, is_latest: bool) -> String {
        let mut url = if is_latest {constants::LATEST_PATH.to_owned()} else {constants::RELEASES_PATH.to_owned()};
        url = url.replacen("{}", constants::PROJECT_OWNER, 1);
        match install_type {
            InstallType::Proton => 
                url.replacen("{}", constants::PROTON_PROJECT_NAME, 1),
            InstallType::Wine =>
                url.replacen("{}", constants::WINE_PROJECT_NAME, 1),
        }
    }

    /* Reqwest has its own error type that seems to be incompatible with anyhow.
     * For the sake of not returning loads of different error types, just convert
     * the reqwest error to an anyhow error. I'm probably doing this poorly
     * and should look into better ways of handling errors
     */
    fn convert_reqwest_error<S, T>(message: S, e: reqwest::Error) -> Result<T, anyhow::Error>
    where
        S: ToString + std::fmt::Display,
    {
        Err(anyhow::anyhow!("{}: {:?}", message, e))
    }

    fn bytes_conversion<'a>(e: u64) -> (u64, &'a str) {
        if e >= 1<<30 { (e/(1<<30), "GB") }
        else if e >= 1<<10 { (e/(1<<30), "MB") }
        else { (e, "B") }
    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn can_get_releases() -> anyhow::Result<()> {
        use crate::github::api::releases;
        use crate::cmd::InstallType;
        let result = releases(InstallType::Proton, Some(50), Some(1))?;
        assert_eq!(result.len(), 50);
        Ok(())
    }

    #[test]
    fn can_get_latest_release() -> anyhow::Result<()> {
        use crate::github::api::latest_release;
        use crate::cmd::InstallType;
        let _result = latest_release(InstallType::Proton)?;
        Ok(())
    }

    #[test]
    fn can_get_release_by_tag() -> anyhow::Result<()> {
        use crate::cmd::InstallType;
        use crate::github::api::{release_version, Release};
        let version: String = String::from("GE-Proton8-4");
        let release: Release = release_version(InstallType::Proton, String::from("GE-Proton8-4"))?;
        assert_eq!(release.tag_name, version);
        Ok(())
    }

    #[test]
    fn can_get_asset_ids() -> anyhow::Result<()> {
        use crate::github::api::{get_asset_ids, release_version, Release};
        use crate::cmd::InstallType;

        let release: Release = release_version(InstallType::Proton, String::from("GE-Proton8-4"))?;
        let ids = get_asset_ids(release)?;
        assert_eq!(ids[0].name, String::from("GE-Proton8-4.tar.gz"));
        assert_eq!(ids[1].name, String::from("GE-Proton8-4.sha512sum"));
        Ok(())
    }
}
