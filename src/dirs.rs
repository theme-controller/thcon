#[cfg(mac)]
use std::env;
use std::path::PathBuf;

#[cfg(mac)]
pub fn config() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME")
        .and_then(|path| {
            let path = PathBuf::from(path);
            if path.is_absolute() {
                Some(path)
            } else {
                None
            }
        })
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
}

#[cfg(mac)]
pub fn data() -> Option<PathBuf> {
    env::var_os("XDG_DATA_HOME")
        .and_then(|path| {
            let path = PathBuf::from(path);
            if path.is_absolute() {
                Some(path)
            } else {
                None
            }
        })
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/share")))
}

#[cfg(not(mac))]
pub fn config() -> Option<PathBuf> {
    ::dirs::config_dir()
}

#[cfg(not(mac))]
pub fn data() -> Option<PathBuf> {
    ::dirs::data_dir()
}

pub fn temp() -> PathBuf {
    #[cfg(not(windows))]
    return PathBuf::from("/tmp");

    #[cfg(windows)]
    todo!();
}
