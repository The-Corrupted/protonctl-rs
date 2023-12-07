use crate::constants;
use dirs::home_dir;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstallType {
    Proton,
    Wine,
}

impl InstallType {
    pub fn get_url(&self, latest: bool) -> String {
        let mut url = if latest {
            constants::LATEST_PATH.to_owned()
        } else {
            constants::RELEASES_PATH.to_owned()
        };

        match self {
            InstallType::Wine => {
                url = url.replace("{1}", constants::PROJECT_OWNER);
                url = url.replace("{2}", constants::WINE_PROJECT_NAME);
            },
            InstallType::Proton => {
                url = url.replace("{1}", constants::PROJECT_OWNER);
                url = url.replace("{2}", constants::PROTON_PROJECT_NAME);
            }
        }
        url
    }

    pub fn get_extension(&self) -> String {
        match self {
            InstallType::Wine => String::from(".tar.xz"),
            InstallType::Proton => String::from(".tar.gz")
        }
    }

    pub fn get_compat_directory_safe(&self) -> anyhow::Result<std::path::PathBuf> {
        let mut compat_dir = home_dir().ok_or(anyhow::anyhow!("Failed to get users home directory"))?;

        let compat_path = match self {
            InstallType::Wine => constants::paths().get(&constants::LockReferences::LutrisRunnersPath).unwrap(),
            InstallType::Proton => constants::paths().get(&constants::LockReferences::SteamCompatPath).unwrap(),
        };
        compat_dir.push(compat_path);
        if !compat_dir.exists() {
            std::fs::create_dir_all(&compat_dir)?;
            Ok(compat_dir)
        } else {
            Ok(compat_dir)
        }
    }
}
