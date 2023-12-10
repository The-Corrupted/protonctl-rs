use std::path::PathBuf;
use std::sync::OnceLock;

pub const MAX_PER_PAGE: u8 = 50;

pub const PROJECT_OWNER: &str = "GloriousEggroll";
pub const WINE_PROJECT_NAME: &str = "wine-ge-custom";
pub const PROTON_PROJECT_NAME: &str = "proton-ge-custom";


pub fn install_path() -> &'static PathBuf {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        PathBuf::from(".local/share/protonctl")
    })
}
