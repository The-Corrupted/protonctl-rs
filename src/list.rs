use crate::cli::InstallTypeCmd;
use crate::cli_utils::Run;
use anyhow::Context;
use async_trait::async_trait;
use console::{Style, Term};
use protonctllib::{
    github::api::Release,
    version_info::{get_installed_versions, get_releases_paged},
};
use std::io::Write;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct List {
    pub number: u8,
    pub page: u8,
    pub local: bool,
    pub install_type: InstallTypeCmd,
}

impl List {
    pub fn new(number: u8, page: u8, local: bool, install_type: InstallTypeCmd) -> Self {
        Self {
            number,
            page,
            local,
            install_type,
        }
    }

    pub fn set_number(&mut self, number: u8) -> &mut Self {
        self.number = number;
        self
    }

    pub fn set_page(&mut self, page: u8) -> &mut Self {
        self.page = page;
        self
    }

    pub fn set_local(&mut self, local: bool) -> &mut Self {
        self.local = local;
        self
    }

    pub fn set_install_type(&mut self, install_type: InstallTypeCmd) -> &mut Self {
        self.install_type = install_type;
        self
    }
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
            change_log_style: Style::new().dim(),
        }
    }
}

#[async_trait]
impl Run for List {
    async fn run(&self) -> anyhow::Result<()> {
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
                print_release(&term, &styles, &release);
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
