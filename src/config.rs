use serde::{Deserialize, Serialize};
use std::fs;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub window: Window,
    pub shader: String,
    pub fps: u64,
}

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("Could not read config file `config.yml`: {0}")]
    ReadFailed(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Error occurred while parsing config file: {0}")]
    YamlParseFailed(
        #[from]
        #[source]
        serde_yaml::Error,
    ),
}

/// Read config file from `config.yml` in cwd.
///
/// # Example
///
/// ```rs
/// let conf = config::read_config()?;
/// ```
pub fn read_config() -> Result<Config, ConfigLoadError> {
    let yaml_reader = fs::File::open("config.yml")?;

    Ok(serde_yaml::from_reader(yaml_reader)?)
}
