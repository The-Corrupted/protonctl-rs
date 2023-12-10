use crate::constants::install_path;
use std::io::Read;
use sha2::{Sha512, Digest};

use anyhow::Context;
use dirs::home_dir;

pub fn get_download_directory_safe() -> anyhow::Result<std::path::PathBuf> {
    let mut install_dir = home_dir().ok_or(anyhow::anyhow!("Couldn't get users home directory"))?;
    install_dir.push(install_path());
    if !install_dir.exists() {
        std::fs::create_dir_all(&install_dir)?;
        Ok(install_dir)
    } else {
        Ok(install_dir)
    }
}

pub fn remove_download_pair(downloaded: &[std::path::PathBuf]) {
    for download in downloaded {
        if std::fs::remove_file(download).is_err() {
            eprintln!("Failed to remove file: {:?}", download);
        }
    }
}

pub fn remove_entry(file: &std::path::PathBuf) -> anyhow::Result<()> {
    if file.is_dir() {
        if std::fs::remove_dir_all(file).is_err() {
            eprintln!("Failed to remove directory {:?}", file);
        }
    } else if std::fs::remove_file(file).is_err() {
        eprintln!("Failed to remove file {:?}", file);
    }
    Ok(())
}

pub fn remove_all_in(path: &std::path::PathBuf) -> anyhow::Result<()> {
    let entries = std::fs::read_dir(path).context("Failed to read directory")?;
    for entry in entries.flatten() {
        let entry_path = entry.path();
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                if std::fs::remove_dir_all(&entry_path).is_err() {
                    eprintln!("Failed to remove directory {:?}", entry_path);
                }
            } else if std::fs::remove_file(&entry_path).is_err() {
                eprintln!("Failed to remove file {:?}", entry_path);
            }
        } else {
            eprintln!("Failed to get file type for {:?}: Skipping", entry_path);
        }
    }
    Ok(())
}

pub fn check_sha(tar: &std::path::PathBuf, sha: &std::path::PathBuf) -> anyhow::Result<bool> {
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(tar)
        .context(format!("Failed to open compressed file: {:?}", tar))?;
    let mut sha_file = std::fs::OpenOptions::new()
        .read(true)
        .open(sha)
        .context(format!("Failed to open sha file: {:?}", sha))?;
    let mut hasher = Sha512::new();
    std::io::copy(&mut file, &mut hasher).context("Failed to copy file contents to hasher")?;
    let final_hash = format!("{:x}", hasher.finalize());
    let mut expected_hash = String::new();
    sha_file.read_to_string(&mut expected_hash)
        .context("Failed to read sha file contents")?;

    match expected_hash.get(0..128) {
        Some(u) => Ok(u == final_hash),
        None => Err(anyhow::anyhow!("Failed to get sha slice"))
    }
}
