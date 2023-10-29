use crate::constants;

pub fn get_compat_directory_safe() -> anyhow::Result<std::path::PathBuf> {
    let mut compat_dir = match constants::HOME_DIR.to_owned() {
        Some(home) => home,
        None => {
            return Err(anyhow::anyhow!("Could not find users home directory"));
        }
    };
    compat_dir.push(constants::STEAM_COMPAT_PATH.clone());
    if !compat_dir.exists() {
        std::fs::create_dir_all(&compat_dir)?;
        println!("Created compatibility tools directory");
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
        println!("Create shared install directory");
        Ok(install_dir)
    } else {
        Ok(install_dir)
    }
}
