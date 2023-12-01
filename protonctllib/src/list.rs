use crate::cmd::{InstallType, Run};
use crate::constants::{paths, LockReferences, MAX_PER_PAGE};
use crate::github;
use crate::colored_out::StdOut;
use anyhow;
use clap::Args;
use dirs::home_dir;
use termcolor::{ColorSpec, Color};

#[derive(Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct List {
    #[arg(short = 'n', long, default_value_t = 10, required = false)]
    pub number: u8,
    #[arg(short = 'p', long, default_value_t = 1, required = false)]
    pub page: u8,
    #[arg(short = 'l', long, default_value_t = false, required = false)]
    pub local: bool,
}

impl Run for List {
    fn run(&self, install_type: InstallType) -> anyhow::Result<()> {
        let mut out = StdOut::new(termcolor::ColorChoice::Always);
        if self.local {
            let versions = get_installed_versions(install_type)?;
            for version in versions {
                let version = version.file_name();
                match version.to_str() {
                    Some(name) => {
                        println!("{}", name);
                    }
                    None => {
                        println!("Something went wrong converting {:?} to a string", version);
                    }
                }
            }
        } else if let Some(releases) = get_releases_paged(install_type, self.number, self.page) {
            for release in releases {
                print_releases_formatted(&mut out, release.tag_name, release.body, release.html_url);
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get releases"));
        }
        print_err(out.flush());
        Ok(())
    }
}

fn print_releases_formatted(out: &mut StdOut, version: String, body: String, url: String) {
    let mut color_spec = ColorSpec::new();
    print_err(out.set_color_spec(color_spec.set_fg(Some(Color::White)).set_bold(true))
        .write("Version: "));
    print_err(out.set_color_spec(color_spec.set_fg(Some(Color::Green)).set_bold(false).set_italic(true))
        .write(format!("{}\n", version)));
    print_err(out.set_color_spec(color_spec.set_fg(Some(Color::White)).set_bold(true).set_italic(false))
        .write("Download: "));
    print_err(out.set_color_spec(color_spec.set_fg(Some(Color::Blue)).set_bold(false).set_italic(true))
        .write(format!("{}\n", url)));
    print_err(out.set_color_spec(color_spec.set_fg(Some(Color::Rgb(96,96,96))).set_italic(false))
                  .write(format!("{}\n\n", body)));
}

fn print_err(result: anyhow::Result<&mut StdOut>) {
    if let Err(e) = result {
        eprintln!("Failed to print colored text: {}", e);
    }
}

pub fn get_releases_paged(
    install_type: InstallType,
    mut number: u8,
    page: u8,
) -> Option<github::api::Releases> {
    if number > MAX_PER_PAGE {
        number = MAX_PER_PAGE
    }

    let releases_wrapped = github::api::releases(install_type, Some(number), Some(page));
    let releases = match releases_wrapped {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    Some(releases)
}

pub fn get_installed_versions(install_type: InstallType) -> anyhow::Result<Vec<std::fs::DirEntry>> {
    let mut home: std::path::PathBuf =
        home_dir().ok_or(anyhow::anyhow!("Couldn't get users home directory"))?;
    let compat_path = match install_type {
        InstallType::Wine => paths().get(&LockReferences::LutrisRunnersPath).unwrap(),
        InstallType::Proton => paths().get(&LockReferences::SteamCompatPath).unwrap(),
    };
    home.push(compat_path);
    let dir_entries_result = std::fs::read_dir(home);
    let mut entries: Vec<std::fs::DirEntry> = Vec::new();
    let dir_entries = match dir_entries_result {
        Ok(d) => d,
        Err(_e) => {
            return Err(anyhow::anyhow!(
                "Failed to read compatibility directory. Does it exist?"
            ));
        }
    };
    for dir in dir_entries {
        match dir {
            Ok(d) => {
                entries.push(d);
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_get_local_dir() -> anyhow::Result<()> {
        use crate::cmd::InstallType;
        use crate::list::get_installed_versions;
        let results = get_installed_versions(InstallType::Proton)?;
        if !results.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Directory was empty"))
        }
    }
}
