
use std::{error::Error, path::PathBuf};
use serde::{Deserialize, Serialize};
use tokio::fs;



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub games_dir: PathBuf,
    pub api_key: String,
}

impl Config {

    fn base_dir() -> Result<PathBuf, Box<dyn Error>> {
        let mut base_dir = std::env::current_exe()?;
        base_dir.pop();
        if cfg!(debug_assertions) {
            base_dir.pop();
            base_dir.pop();
        }
        Ok(base_dir)
    }

    pub async fn load_from_file(file: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut config: Config = serde_json::from_str(&fs::read_to_string(file).await?)?;

        // Fix config paths
        if !config.games_dir.has_root() {
            let mut new_games_dir = Self::base_dir()?;
            new_games_dir.push(&config.games_dir);
            config.games_dir = new_games_dir;
        }

        Ok(config)
    }

    pub async fn load() -> Result<Self, Box<dyn Error>> {
        // Locate the config file.
        let mut config_location = Self::base_dir()?;
        config_location.push("itch-io-downloader.json");

        Ok(Self::load_from_file(config_location).await?)
    }

}


