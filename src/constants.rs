use lazy_static::lazy_static;
use dirs::home_dir;

pub const MAX_PER_PAGE: u8 = 50;
pub const PROTON_GE_RELEASE_PATH: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";
pub const PROTON_GE_LATEST_PATH: &str = "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest";

lazy_static! {
    pub static ref HOME_DIR: Option<std::path::PathBuf> = home_dir();
    pub static ref STEAM_COMPAT_PATH: std::path::PathBuf = std::path::PathBuf::from(".steam/root/compatibilitytools.d");
}
