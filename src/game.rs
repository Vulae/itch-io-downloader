#![allow(dead_code)]

use std::{error::Error, fs::DirEntry, path::PathBuf};
use serde::{Deserialize, Serialize};
use tokio::{fs, process::Command};

use crate::{config::Config, api::{itch_api_game_uploads, GameUpload}};



#[derive(Deserialize, Serialize)]
pub struct GameJson {
    pub game_id: i64,
    pub upload_id: i64,
    pub title: String,
    pub description: String,
    pub url: String,
    pub directory: String,
}



#[derive(Debug, Clone)]
pub struct Game {
    pub config: Config,
    pub game_id: i64,
    pub upload_id: i64,
    pub title: String,
    pub description: String,
    pub url: String,
    pub directory: String,
}



impl Game {

    pub fn new(config: Config, game_json: &GameJson) -> Self {
        Self {
            config,
            game_id: game_json.game_id,
            upload_id: game_json.upload_id,
            title: game_json.title.clone(),
            description: game_json.description.clone(),
            url: game_json.url.clone(),
            directory: game_json.directory.clone()
        }
    }

    pub fn json(&mut self) -> GameJson {
        GameJson {
            game_id: self.game_id,
            upload_id: self.upload_id,
            title: self.title.clone(),
            description: self.description.clone(),
            url: self.url.clone(),
            directory: self.directory.clone(),
        }
    }



    pub async fn is_downloaded(&mut self) -> Result<bool, Box<dyn Error>> {
        let mut game_path = PathBuf::from(&self.config.base_dir);
        game_path.push("games");
        game_path.push(&self.directory);

        if !fs::try_exists(&game_path).await? {
            return Ok(false);
        }

        let meta = fs::metadata(&game_path).await?;

        Ok(meta.is_dir())
    }

    pub async fn is_latest(&mut self) -> Result<bool, Box<dyn Error>> {
        if !self.is_downloaded().await? {
            return Ok(false);
        }

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

        let mut search_path = PathBuf::from(&self.config.base_dir);
        search_path.push("games");
        search_path.push(&self.directory);
        let executable_path = find_executable(search_path)?;

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





static SEARCH_BLACKLIST: &'static [&'static str] = &[
    "UnityCrashHandler64.exe",
    "UnityCrashHandler32.exe" // Don't actually know if this exists but include it just incase.
];

fn is_executable_blacklisted(path: &PathBuf) -> bool {
    SEARCH_BLACKLIST.iter().any(|blacklisted| {
        path.to_str().unwrap().ends_with(blacklisted)
    })
}

fn find_executable_parent(path: PathBuf) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let mut queue: Vec<PathBuf> = vec![ path ];

    while queue.len() > 0 {
        let path = queue.remove(0);

        if is_executable_blacklisted(&path) {
            continue;
        }

        if path.is_dir() {
            for entry in path.read_dir()? {
                queue.push(entry?.path());
            }
        } else if path.is_file() {
            if path.extension().is_some() && path.extension().unwrap() == "exe" {
                return Ok(match path.parent() {
                    Some(parent) => Some(PathBuf::from(parent)),
                    None => None,
                });
            }
        }
    }

    Ok(None)
}

fn find_executable(path: PathBuf) -> Result<Option<PathBuf>, Box<dyn Error>> {
    match find_executable_parent(path)? {
        Some(executable_dir) => {

            let mut executables: Vec<DirEntry> = executable_dir.read_dir()?
                .filter(|item| {
                    match item {
                        Ok(item) => {
                            let path = item.path();
                            return path.is_file() && 
                                !is_executable_blacklisted(&path) &&
                                (path.extension().is_some() && path.extension().unwrap() == "exe");
                        },
                        Err(_) => false,
                    }
                })
                .map(|item| item.unwrap())
                .collect::<Vec<DirEntry>>();

            executables.sort_by(|a, b| {
                // Use x64 executables over x32
                if a.file_name().to_str().unwrap().contains("64") { return std::cmp::Ordering::Greater; }
                if b.file_name().to_str().unwrap().contains("64") { return std::cmp::Ordering::Less; }
                
                // Otherwise biggest is probably the correct executable
                let a_meta = a.metadata().unwrap();
                let b_meta = b.metadata().unwrap();
                a_meta.len().cmp(&b_meta.len())
            });

            Ok(Some(executables[0].path()))

        },
        None => Ok(None)
    }
}


