use crate::cmd::InstallTypeCmd;
use anyhow::Context;
use clap::Args;
use console::{Style, Term};
use protonctllib::{
    github::api::Release,
    version_info::{get_installed_versions, get_releases_paged},
};
use std::io::Write;

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


struct Styles {
    prefix_style: Style,
    version_style: Style,
    url_style: Style,
    change_log_style: Style,
}

impl Styles {
    pub fn new() -> Self {
        Self {
            prefix_style: Style::new().bold(),
            version_style: Style::new().green(),
            url_style: Style::new().blue().underlined(),
            change_log_style: Style::new().dim()
        }
    }
}

// We need to do output stuff here.
impl List {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut term = Term::buffered_stdout();
        if self.local {
            let style = Style::new().blue();
            let mut iters = 1;
            let versions = get_installed_versions(
                &self
                    .install_type
                    .get_compat_directory_safe()
                    .context("Failed to get compatibility directory")?,
            )
            .context("Failed to get directory entries")?;
            for version in versions {
                let version = version.file_name();
                if let Some(name) = version.to_str() {
                    let mut name = name.to_string();
                    name.push_str("   ");
                    term.write_fmt(format_args!("{}", style.apply_to(name)))
                        .unwrap();
                    if iters % 3 == 0 {
                        term.write_all(b"\n").unwrap();
                    }
                } else {
                    eprintln!("Failed to convert file_name to string");
                }
                iters += 1;
            }
            term.write_all(b"\n").unwrap();
        } else if let Some(releases) =
            get_releases_paged(self.install_type.get_url(false), self.number, self.page).await
        {
            let styles = Styles::new();
            for release in releases {
                print_release(&mut term, &styles, &release);
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get releases"));
        }
        term.flush().unwrap();
        Ok(())
    }
}

fn print_release(term: &Term, styles: &Styles, release: &Release) {
    term.write_line(
        format!(
            "{}: {}",
            styles.prefix_style.apply_to("Version"),
            styles.version_style.apply_to(&release.tag_name)
        )
        .as_str(),
    )
    .unwrap();
    term.write_line(
        format!(
            "{}: {}",
            styles.prefix_style.apply_to("Url"),
            styles.url_style.apply_to(&release.html_url)
        )
        .as_str(),
    )
    .unwrap();
    term.write_line(format!("{}\n", styles.change_log_style.apply_to(&release.body)).as_str())
        .unwrap();
}
