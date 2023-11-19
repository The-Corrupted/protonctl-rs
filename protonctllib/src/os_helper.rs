use crate::constants;
use crate::cmd::InstallType;

pub fn get_compat_directory_safe(install_type: InstallType) -> anyhow::Result<std::path::PathBuf> {
    let mut compat_dir = match constants::HOME_DIR.to_owned() {
        Some(home) => home,
        None => {
            return Err(anyhow::anyhow!("Could not find users home directory"));
        }
    };
    let compat_path = match install_type {
        InstallType::Wine => constants::LUTRIS_RUNNERS_PATH.to_owned(),
        InstallType::Proton => constants::STEAM_COMPAT_PATH.to_owned(),
    };
    compat_dir.push(compat_path);
    if !compat_dir.exists() {
        std::fs::create_dir_all(&compat_dir)?;
        Ok(compat_dir)
    } else {
        Ok(compat_dir)
    }
}

pub fn get_install_directory_safe() -> anyhow::Result<std::path::PathBuf> {
    let mut install_dir = match constants::HOME_DIR.to_owned() {
        Some(home) => home,
        None => {
            return Err(anyhow::anyhow!("Could not find users home directory"));
        }
    };
    install_dir.push(constants::INSTALL_PATH.clone());
    if !install_dir.exists() {
        std::fs::create_dir_all(&install_dir)?;
        Ok(install_dir)
    } else {
        Ok(install_dir)
    }
}
