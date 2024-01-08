#![allow(dead_code)]

use std::{path::PathBuf, error::Error};
use tokio::{fs, process::Command};

use crate::{config::Config, api::{itch_api_game_uploads, GameUpload}};



static SEARCH_BLACKLIST: &'static [&'static str] = &[
    "UnityCrashHandler64.exe"
];



#[derive(Debug)]
pub struct Game {
    config: Config,
    path: PathBuf,
    game_id: i64,
    upload_id: i64,
}



impl Game {

    pub fn new(config: Config, path: PathBuf, id: i64, version: i64) -> Self {
        Self { config, path, game_id: id, upload_id: version }
    }

    pub async fn is_latest(&mut self) -> Result<bool, Box<dyn Error>> {
        let mut game_uploads = itch_api_game_uploads(&self.config.api_key, &self.game_id).await?.uploads;
        game_uploads.sort_by(|a, b| {
            a.id.cmp(&b.id)
        });
        let game_upload = game_uploads.iter()
            .filter(|game_upload| game_upload.p_windows)
            .collect::<Vec<&GameUpload>>().first().unwrap().to_owned();

        Ok(game_upload.id <= self.upload_id)
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {

        fn search_exe(path: PathBuf) -> Result<Option<PathBuf>, Box<dyn Error>> {
            let mut queue: Vec<PathBuf> = vec![ path ];
        
            while queue.len() > 0 {
                let path = queue.remove(0);
        
                if SEARCH_BLACKLIST.iter().any(|blacklisted| {
                    path.to_str().unwrap().ends_with(blacklisted)
                }) {
                    continue;
                }
        
                if path.is_dir() {
                    for entry in path.read_dir()? {
                        queue.push(entry?.path());
                    }
                } else if path.is_file() {
                    if path.extension().is_some() && path.extension().unwrap() == "exe" {
                        return Ok(Some(path.to_owned()));
                    }
                }
            }
        
            Ok(None)
        }

        let executable_path = search_exe(PathBuf::from(&self.path))?;

        if executable_path.is_some() {
            let full_path = fs::canonicalize(executable_path.unwrap()).await?;
            // I'm dumb and don't know how canonicalize works, it adds \\?\ to beginning?
            let (_, exec_path) = full_path.to_str().unwrap().split_at(4);
            let _program = Command::new(exec_path).spawn().unwrap();
        } else {
            return Err("Failed to find executable.".into());
        }

        Ok(())
    }

}
