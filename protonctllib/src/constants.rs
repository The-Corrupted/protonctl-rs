use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

pub const MAX_PER_PAGE: u8 = 50;

pub const PROJECT_OWNER: &str = "GloriousEggroll";
pub const WINE_PROJECT_NAME: &str = "wine-ge-custom";
pub const PROTON_PROJECT_NAME: &str = "proton-ge-custom";
pub const RELEASES_PATH: &str = "https://api.github.com/repos/{1}/{2}/releases";
pub const LATEST_PATH: &str = "https://api.github.com/repos/{1}/{2}/releases/latest";

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum LockReferences {
    SteamCompatPath,
    LutrisRunnersPath,
    InstallPath,
}

pub fn paths() -> &'static HashMap<LockReferences, PathBuf> {
    static PATHS: OnceLock<HashMap<LockReferences, PathBuf>> = OnceLock::new();
    PATHS.get_or_init(|| {
        HashMap::from([
            (
                LockReferences::LutrisRunnersPath,
                PathBuf::from(".local/share/lutris/runners/wine"),
            ),
            (
                LockReferences::SteamCompatPath,
                PathBuf::from(".local/share/Steam/compatibilitytools.d"),
            ),
            (
                LockReferences::InstallPath,
                PathBuf::from(".local/share/protonctl"),
            ),
        ])
    })
}
