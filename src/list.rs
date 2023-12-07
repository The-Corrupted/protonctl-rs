use clap::Args;
use std::io::Write;
use console::{Term, Style};
use anyhow::Context;
use crate::cmd::InstallTypeCmd;
use protonctllib::{version_info::{get_releases_paged, get_installed_versions}, github::api::Release};

#[derive(Args, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct List {
    #[arg(value_enum, default_value_t = InstallTypeCmd::Proton, required = false)]
    install_type: InstallTypeCmd,
    #[arg(long = "number", short = 'n', required = false, default_value_t = 10)]
    pub number: u8,
    #[arg(long = "page", short = 'p', required = false, default_value_t = 1)]
    pub page: u8,
    #[arg(short = 'l', required = false, default_value_t = false)]
    pub local: bool,
}

// We need to do output stuff here.
impl List {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut term = Term::buffered_stdout();
        if self.local {
            let style = Style::new().green();
            let mut iters = 1;
            let versions = get_installed_versions(self.install_type.get_compat_directory_safe()
                                                  .context("Failed to get compatibility directory")?)
                .context("Failed to get directory entries")?;
            for version in versions {
                let version = version.file_name();
                if let Some(name) = version.to_str() {
                    let mut name = name.to_string();
                    name.push_str("   ");
                    term.write_fmt(format_args!("{}", style.apply_to(name))).unwrap();
                    if iters % 3 == 0 {
                        term.write(b"\n").unwrap();
                    }
                } else {
                    eprintln!("Failed to convert file_name to string");
                }
                iters += 1;
            }
            term.write(b"\n").unwrap();
        } else if let Some(releases) = get_releases_paged(self.install_type.get_url(false), self.number, self.page).await {
            for release in releases {
                print_release(&mut term, release);
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get releases"));
        }
        term.flush().unwrap();
        Ok(())
    }
}

fn print_release(term: &mut Term, release: Release) {
    let bold = Style::new().bold();
    let link = Style::new().underlined().blue().bright();
    let change_log = Style::new().dim();
    let version = Style::new().italic().green();
    term.write_line(format!("{}: {}", bold.apply_to("Version"), version.apply_to(release.tag_name)).as_str()).unwrap();
    term.write_line(format!("{}: {}", bold.apply_to("Url"), link.apply_to(release.html_url)).as_str()).unwrap();
    term.write_line(format!("{}\n", change_log.apply_to(release.body)).as_str()).unwrap();
}
