
use std::{path::PathBuf, error::Error};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::{fs::{self, File}, io::{AsyncWriteExt, AsyncReadExt}};
use zip::ZipArchive;
use crate::{game::Game, config::Config, api::{GameUpload, itch_api_game_uploads, itch_api_upload_download}};



#[derive(Deserialize, Serialize)]
struct GameInfo {
    game_id: i64,
    upload_id: i64,
}



#[derive(Debug)]
pub struct Library {
    config: Config,
    path: PathBuf,
}



impl Library {

    pub fn new(config: Config, path: PathBuf) -> Self {
        Self { config, path }
    }

    fn game_path(&mut self, game_id: i64) -> PathBuf {
        let mut game_path = PathBuf::from(&self.path);
        game_path.push(game_id.to_string());
        game_path
    }

    fn game_info_path(&mut self, game_id: i64) -> PathBuf {
        let mut game_info_path = PathBuf::from(&self.path);
        game_info_path.push(format!("{}.json", game_id));
        game_info_path
    }

    fn temp_path(&mut self) -> PathBuf {
        let mut temp_path = PathBuf::from(&self.path);
        temp_path.push("temp");
        temp_path
    }

    pub async fn create_dirs(&mut self) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&self.path).await?;
        fs::create_dir_all(&self.temp_path()).await?;
        Ok(())
    }

    pub async fn download_game(&mut self, game_id: i64) -> Result<Game, Box<dyn Error>> {
        // Get latest upload.
        let mut game_uploads = itch_api_game_uploads(&self.config.api_key, &game_id).await?.uploads;
        game_uploads.sort_by(|a, b| {
            a.id.cmp(&b.id)
        });
        let game_upload = game_uploads.iter()
            .filter(|game_upload| game_upload.p_windows)
            .collect::<Vec<&GameUpload>>().first().unwrap().to_owned();

        // Upload link to download.
        let game_download = itch_api_upload_download(&self.config.api_key, &game_upload.id).await?;
        
        // Download
        let mut temp_path = self.temp_path();
        temp_path.push(format!("{}.zip", game_id));
        let mut temp_file = File::create(&temp_path).await?;
        let mut stream = reqwest::get(game_download.url).await?.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            temp_file.write_all(&chunk).await?;
        }
        temp_file.flush().await?;

        // Unzip file
        let game_path = self.game_path(game_id);
        fs::create_dir_all(&game_path).await?;
        let zip_file = std::fs::File::open(&temp_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        archive.extract(&game_path)?;

        // Cleanup temp
        fs::remove_file(temp_path).await?;

        // Add game json
        let game_info_path = self.game_info_path(game_id);
        let mut game_info_file = fs::File::create(game_info_path).await?;
        let game_info = GameInfo {
            game_id: game_id.clone(),
            upload_id: game_upload.id.clone()
        };
        game_info_file.write(serde_json::to_string(&game_info)?.as_bytes()).await?;
        game_info_file.flush().await?;

        Ok(self.get_game(game_id).await?.unwrap())
    }

    pub async fn get_game(&mut self, game_id: i64) -> Result<Option<Game>, Box<dyn Error>> {
        let game_path = self.game_path(game_id);
        let game_info_path = self.game_info_path(game_id);

        let game_info_file = File::open(&game_info_path).await;
        if game_info_file.is_err() {
            return Ok(None);
        }

        let mut game_info_str = String::new();
        game_info_file?.read_to_string(&mut game_info_str).await?;
        let game_info: GameInfo = serde_json::from_str(&game_info_str)?;
        if game_info.game_id != game_id {
            return Err("Game id & game id in game info do not match.".into());
        }

        Ok(Some(Game::new(
            self.config.clone(),
            game_path,
            game_info.game_id,
            game_info.upload_id
        )))
    }

}


