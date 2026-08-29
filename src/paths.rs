use std::env;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

pub fn cache_dir() -> PathBuf {
    if let Some(path) = env::var_os("OKMATE_CACHE") {
        return PathBuf::from(path);
    }
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".okmate")
        .join("cache")
}

pub fn state_dir() -> PathBuf {
    if let Some(path) = env::var_os("OKMATE_STATE") {
        return PathBuf::from(path);
    }
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".okmate")
        .join("state")
}

pub fn config_path() -> PathBuf {
    if let Ok(path) = env::var("OKMATE_CONFIG")
        && !path.is_empty()
    {
        return PathBuf::from(path);
    }
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".okmate")
        .join("config.toml")
}
