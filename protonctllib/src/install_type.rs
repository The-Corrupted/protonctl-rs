use crate::constants;
use dirs::home_dir;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstallType {
    Proton,
    Wine,
}

impl InstallType {
    // Figure out a better way to do this. We are more or less doing the same thing
    // 4 times
    pub fn get_url(&self, latest: bool) -> String {
        let mut url: String = String::new();
        match self {
            InstallType::Wine => {
                url = if latest {
                    format!("https://api.github.com/repos/{}/{}/releases/latest",
                            constants::PROJECT_OWNER, constants::WINE_PROJECT_NAME)
                } else {

                    format!("https://api.github.com/repos/{}/{}/releases",
                              constants::PROJECT_OWNER, constants::WINE_PROJECT_NAME)
                }
            },
            InstallType::Proton => {
                url = if latest {
                    format!("https://api.github.com/repos/{}/{}/releases/latest",
                           constants::PROJECT_OWNER, constants::PROTON_PROJECT_NAME)
                } else {
                    format!("https://api.github.com/repos/{}/{}/releases",
                            constants::PROJECT_OWNER, constants::PROTON_PROJECT_NAME)
                }
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
            InstallType::Wine => std::path::PathBuf::from(".local/share/lutris/runners/wine"),
            InstallType::Proton => std::path::PathBuf::from(".local/share/Steam/compatibilitytools.d"),
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
