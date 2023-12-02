use std::io::Read;

use crate::cmd::InstallType;
use crate::github::api::{download_assets, get_asset_ids, release_version, Release};
use crate::os_helper::{get_compat_directory_safe, remove_download_pair};
use crate::colored_out::StdOutStream;
use crate::decompress;
use anyhow::Context;
use clap::Args;
use termcolor::{ColorChoice, ColorSpec, Color};
use sha2::{Digest, Sha512};

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Install {
    install_version: String,
}

impl Install {
    pub async fn run(&self, install_type: InstallType) -> anyhow::Result<()> {
        let mut spec = ColorSpec::new();
        let mut out = StdOutStream::new(ColorChoice::Always);
        let compat_directory: std::path::PathBuf = get_compat_directory_safe(install_type)?;
        let release: Release = release_version(install_type, self.install_version.clone()).await?;
        let assets = get_asset_ids(install_type, release)?;
        let downloaded = download_assets(install_type, assets).await?;
        // Newline isn't created between the progress bar. Until I've detatched printing from the
        // rest of the library, just add a newline
        out.write("\n").flush();
        self.check_sha(&mut out, &downloaded)?;
        out.set_color_spec(spec.set_bold(true))
            .write("Decompressing ... ").flush();
        match install_type {
            InstallType::Wine => decompress::lzma(downloaded[0].clone(), compat_directory)?,
            InstallType::Proton => decompress::gunzip(downloaded[0].clone(), compat_directory)?,
        }
        out.set_color_spec(&spec.set_fg(Some(Color::Green)).set_italic(true).set_bold(true))
            .write("Success\n").flush();
        out.set_color_spec(&spec.set_fg(Some(Color::White)).set_italic(false))
                           .write("Removing artifacts\n").flush();
        remove_download_pair(&downloaded);
        Ok(())
    }
}

impl Install {
    pub fn new(install_version: impl ToString) -> Self {
        Self {
            install_version: install_version.to_string(),
        }
    }

    pub fn check_sha(&self, out: &mut StdOutStream, file_locations: &[std::path::PathBuf; 2]) -> anyhow::Result<()> {
        let mut spec = ColorSpec::new();
        out.set_color_spec(spec.set_bold(true)).write("Checking hash ... ").flush()
            .set_color_spec(spec.set_fg(Some(Color::Green)).set_italic(true));
        // Open the tar file and pass it into the hasher
        let mut file_location = file_locations[0].clone();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(&file_location)
            .context(format!(
                "Failed to open compressed file: {:?}",
                file_location
            ))?;
        let mut hasher = Sha512::new();
        std::io::copy(&mut file, &mut hasher).context("Failed to copy file contents to hasher")?;
        let hash = format!("{:x}", hasher.finalize());
        // Get the expected hash from the downloaded sha file.
        file_location = file_locations[1].clone();
        file = std::fs::OpenOptions::new()
            .read(true)
            .open(&file_location)
            .context(format!("Failed to open sha file: {:?}", file_location))?;
        let mut expected_hash = String::new();
        file.read_to_string(&mut expected_hash)
            .context("Failed to read file into string")?;
        expected_hash = expected_hash
            .get(0..128)
            .context("Failed to hash value")?
            .into();
        if expected_hash != hash {
            // Print failure message and return an error
            out.set_color_spec(spec.set_fg(Some(Color::Red)));
            out.write("Fail\n").flush();
            return Err(anyhow::anyhow!(
                "Hash mismatch\n{}\n{}",
                expected_hash,
                hash
            ));
        }
        out.write("Success\n").flush();
        out.set_color_spec(&ColorSpec::new());
        Ok(())
    }
}
