
use std::{path::PathBuf, error::Error};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::{fs::{self, File}, io::AsyncWriteExt};
use crate::{game::{Game, GameJson}, config::Config, api::{GameUpload, itch_api_game_uploads, itch_api_upload_download, itch_api_game_info}, utils::extract_archive};



#[derive(Deserialize, Serialize)]
pub struct LibraryJson {
    pub games: Vec<GameJson>,
}



#[derive(Debug, Clone)]
pub struct Library {
    pub config: Config,
    pub games: Vec<Game>,
}



impl Library {

    pub fn new(config: Config, library_json: &LibraryJson) -> Self {
        Self {
            config: config.clone(),
            games: library_json.games.iter().map(|game_json| {
                Game::new(config.clone(), game_json)
            }).collect()
        }
    }

    pub fn json(&mut self) -> LibraryJson {
        LibraryJson {
            games: self.games.iter().map(|game| game.clone().json()).collect()
        }
    }

    pub async fn load(config: Config) -> Result<Library, Box<dyn Error>> {
        let mut games_path = PathBuf::from(&config.base_dir);
        games_path.push("games");
        fs::create_dir_all(&games_path).await?;

        let mut library_info_path = PathBuf::from(&games_path);
        library_info_path.push("library_info.json");
        let library_json: LibraryJson = if library_info_path.is_file() {
            serde_json::from_str::<LibraryJson>(&fs::read_to_string(library_info_path).await?)?
        } else {
            LibraryJson { games: vec![] }
        };

        Ok(Self::new(config, &library_json))
    }

    pub async fn save(&mut self) -> Result<(), Box<dyn Error>> {
        let mut library_info_path = PathBuf::from(&self.config.base_dir);
        library_info_path.push("games/library_info.json");
        let mut library_info_file = fs::File::create(library_info_path).await?;
        library_info_file.write(serde_json::to_string_pretty::<LibraryJson>(&self.json())?.as_bytes()).await?;
        library_info_file.flush().await?;

        Ok(())
    }



    pub fn get_game(&mut self, game_id: &i64) -> Option<&Game> {
        self.games.iter().find(|game| { game.game_id.eq(game_id) })
    }

    pub fn set_game(&mut self, game: &Game) {
        match self.games.iter().enumerate().find(|(_, g)| { g.game_id.eq(&game.game_id) }) {
            Some((i, _)) => { self.games.remove(i); },
            _ => { },
        }

        self.games.push(game.clone());
    }



    pub async fn download_game(&mut self, game_id: i64) -> Result<(), Box<dyn Error>> {
        // Get game info.
        let game_info = itch_api_game_info(&self.config.api_key, &game_id).await?.game;
        if game_info.id != game_id {
            return Err("Game id does not match.".into());
        }

        // Get latest upload.
        let mut game_uploads = itch_api_game_uploads(&self.config.api_key, &game_info.id).await?.uploads;
        game_uploads.sort_by(|a, b| {
            a.id.cmp(&b.id)
        });
        let game_upload = game_uploads.iter()
            .filter(|game_upload| game_upload.p_windows)
            .collect::<Vec<&GameUpload>>().first().unwrap().to_owned();

        // Upload link to download.
        let game_download = itch_api_upload_download(&self.config.api_key, &game_upload.id).await?;
        
        // Download game.
        let mut temp_path = PathBuf::from(&self.config.base_dir);
        temp_path.push("games/temp");
        fs::create_dir_all(&temp_path).await?;
        temp_path.push(&game_upload.filename);
        let mut temp_file = File::create(&temp_path).await?;
        let mut stream = reqwest::get(game_download.url).await?.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            temp_file.write_all(&chunk).await?;
        }
        temp_file.flush().await?;

        // Extract game archive.
        let mut games_path = PathBuf::from(&self.config.base_dir);
        games_path.push("games");
        let mut game_path = PathBuf::from(&games_path);
        game_path.push(game_info.id.to_string());
        extract_archive(&temp_path, &game_path).await?;

        // Cleanup temp
        fs::remove_file(&temp_path).await?;

        // Add game to self
        self.set_game(&Game::new(self.config.clone(), &GameJson {
            game_id: game_info.id,
            upload_id: game_upload.id,
            title: game_info.title,
            description: game_info.short_text,
            url: game_info.url,
            directory: game_path.strip_prefix(&games_path)?.to_str().unwrap().into(),
        }));

        // Update library_info.json
        self.save().await?;

        Ok(())
    }

}


