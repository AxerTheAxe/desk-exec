use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    result,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use xdg::BaseDirectories;

pub type Result<T, E = ConfigError> = result::Result<T, E>;

#[derive(Deserialize, Serialize)]
pub struct Config {
    search: Search,
}

impl Config {
    pub fn try_new(file_path: Option<&Path>) -> Result<Self> {
        if let Some(path) = file_path {
            return Self::try_from(path);
        }

        match get_default_config_file() {
            Ok(config) => Self::try_from(config.as_ref()),
            Err(_) => Self::try_from(create_default_config_file()?.as_ref()),
        }
    }

    pub fn populate_dirs(&mut self) -> Option<()> {
        if !self.search.xdg_default_dirs {
            return None;
        }

        if let Some(paths) = desk_exec::get_default_entry_dirs() {
            self.search.dirs.extend(paths);
            Some(())
        } else {
            None
        }
    }

    pub fn get_dirs(&self) -> Option<impl Iterator<Item = &PathBuf>> {
        let filtered = self
            .search
            .dirs
            .iter()
            .filter(|path| path.exists() && !path.as_os_str().is_empty());

        if filtered.clone().next().is_some() {
            Some(filtered)
        } else {
            None
        }
    }

    pub fn clean_dirs(&mut self) {
        self.search
            .dirs
            .retain(|path| !path.as_path().to_str().map_or(true, |path| path.is_empty()));

        self.search.dirs.retain(|path| path.exists())
    }
}

impl TryFrom<&Path> for Config {
    type Error = ConfigError;

    fn try_from(path: &Path) -> result::Result<Self, Self::Error> {
        let file_contents =
            fs::read_to_string(path).map_err(|_| ConfigError::Read(path.to_path_buf()))?;

        let config =
            toml::from_str(&file_contents).map_err(|_| ConfigError::Toml(path.to_path_buf()))?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search: Search {
                xdg_default_dirs: true,
                dirs: Vec::new(),
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Search {
    xdg_default_dirs: bool,
    dirs: Vec<PathBuf>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read from a config file at '{0}'")]
    Read(PathBuf),

    #[error("failed to write to a config file at '{0}'")]
    Write(PathBuf),

    #[error("failed to parse '{0}' into TOML")]
    Toml(PathBuf),

    #[error("failed to get XDG base data directories")]
    Xdg,
}

pub fn create_default_config_file() -> Result<PathBuf> {
    let config_file_location = get_default_config_location().map_err(|_| ConfigError::Xdg)?;

    let config = toml::to_string(&Config::default())
        .map_err(|_| ConfigError::Toml(config_file_location.clone()))?;

    fs::write(&config_file_location, config)
        .map_err(|_| ConfigError::Write(config_file_location.clone()))?;

    println!(
        "Created a config file at '{}'",
        config_file_location.display()
    );

    Ok(config_file_location)
}

fn get_default_config_file() -> anyhow::Result<PathBuf> {
    let config_file_location = get_default_config_location()?;
    File::open(&config_file_location)?;
    Ok(config_file_location)
}

fn get_default_config_location() -> anyhow::Result<PathBuf> {
    let base_directories = BaseDirectories::with_prefix("desk-exec")?;
    let config_file = base_directories.place_config_file("desk_exec.toml")?;
    Ok(config_file)
}
