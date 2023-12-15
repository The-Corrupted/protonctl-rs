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
            self.list_local(&mut term)?;
        } else {
            self.list_remote(&mut term).await?;
        }
        term.flush().unwrap();
        Ok(())
    }
}

impl List {
    fn list_local(&self, term: &mut Term) -> anyhow::Result<()> {
        let directory_style = Style::new().blue();
        let style_header = Style::new().bold().underlined();
        let versions = get_installed_versions(
            &self
                .install_type
                .get_compat_directory_safe()
                .context("Failed to get compatibility directory")?,
        )
        .context("Failed to get directory entries")?;
        let header_str = style_header
            .apply_to(format!("{} installs:", &self.install_type))
            .to_string();
        term.write_line(&header_str).unwrap();
        for version in versions {
            let version = version.file_name();
            if let Some(name) = version.to_str() {
                let name = name.to_string();
                term.write_fmt(format_args!("{}\n", directory_style.apply_to(name)))
                    .unwrap();
            } else {
                eprintln!("Failed to convert file_name to string");
            }
        }
        Ok(())
    }

    async fn list_remote(&self, term: &mut Term) -> anyhow::Result<()> {
        if let Some(releases) =
            get_releases_paged(self.install_type.get_url(false), self.number, self.page).await
        {
            let styles = Styles::new();
            for release in releases {
                print_release(term, &styles, &release);
            }
        } else {
            return Err(anyhow::anyhow!("Failed to get releases"));
        }
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
