
use std::{error::Error, path::PathBuf};
use tokio::fs;



#[derive(Clone, Debug)]
pub struct Config {
    pub base_dir: PathBuf,
    pub api_key: String,
}

impl Config {

    pub fn new(base_dir: PathBuf, api_key: String) -> Self {
        Self { base_dir, api_key }
    }

    pub async fn load(base_dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut api_key_path = PathBuf::from(&base_dir);
        api_key_path.push("api_key.txt");
        let api_key = fs::read_to_string(api_key_path).await?;

        Ok(Self::new(base_dir, api_key))
    }

}


