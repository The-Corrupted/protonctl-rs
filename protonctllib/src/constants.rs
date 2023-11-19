use dirs::home_dir;
use lazy_static::lazy_static;

pub const MAX_PER_PAGE: u8 = 50;

pub const PROJECT_OWNER: &str = "GloriousEggroll";
pub const WINE_PROJECT_NAME: &str = "wine-ge-custom";
pub const PROTON_PROJECT_NAME: &str = "proton-ge-custom";
pub const RELEASES_PATH: &str =
    "https://api.github.com/repos/{}/{}/releases";
pub const LATEST_PATH: &str =
    "https://api.github.com/repos/{}/{}/releases/latest";

lazy_static! {
    pub static ref HOME_DIR: Option<std::path::PathBuf> = home_dir();
    pub static ref STEAM_COMPAT_PATH: std::path::PathBuf =
        std::path::PathBuf::from(".local/share/Steam/compatibilitytools.d");
    pub static ref LUTRIS_RUNNERS_PATH: std::path::PathBuf =
        std::path::PathBuf::from(".local/share/lutris/runners/wine");
    pub static ref INSTALL_PATH: std::path::PathBuf =
        std::path::PathBuf::from(".local/share/protonctl");
}
