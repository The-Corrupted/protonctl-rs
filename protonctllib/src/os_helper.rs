use crate::constants;
use crate::cmd::InstallType;

use anyhow::Context;

pub fn get_compat_directory_safe(install_type: InstallType) -> anyhow::Result<std::path::PathBuf> {
    let mut compat_dir = constants::HOME_DIR.to_owned()
        .context("Failed to get users home directory")?;

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

pub fn get_download_directory_safe() -> anyhow::Result<std::path::PathBuf> {
    let mut install_dir = constants::HOME_DIR.to_owned()
        .context("Could not find users home directory")?;
    install_dir.push(constants::INSTALL_PATH.clone());
    if !install_dir.exists() {
        std::fs::create_dir_all(&install_dir)?;
        Ok(install_dir)
    } else {
        Ok(install_dir)
    }
}

pub fn remove_download_pair(downloaded: &[std::path::PathBuf]) {
    for download in downloaded {
        match std::fs::remove_file(&download) {
            Err(_) => println!("Failed to remove file: {:?}", download),
            _ => ()
        }
    }
}

pub fn remove_entry(file: &std::path::PathBuf) -> anyhow::Result<()> {
    if file.is_dir() {
        if std::fs::remove_dir_all(&file).is_err() {
            println!("Failed to remove directory {:?}", file);
        }
    } else {
        if std::fs::remove_file(&file).is_err() {
            println!("Failed to remove file {:?}", file); 
        }
    }
    Ok(())
}

pub fn remove_all_in(path: &std::path::PathBuf) -> anyhow::Result<()> {
    let entries = std::fs::read_dir(path).context("Failed to read directory")?;
    for entry in entries {
        if let Ok(item) = entry {
            let entry_path = item.path();
            if let Ok(file_type) = item.file_type() {
                if file_type.is_dir() {
                    if std::fs::remove_dir_all(&entry_path).is_err() {
                        println!("Failed to remove directory {:?}", entry_path);
                    }
                } else {
                    if std::fs::remove_file(&entry_path).is_err() {
                        println!("Failed to remove file {:?}", entry_path);
                    }
                }
            } else {
                println!("Failed to get file type for {:?}: Skipping", entry_path);
            }
        }
    }
    Ok(())
}
