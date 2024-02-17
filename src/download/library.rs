
use std::{error::Error, fmt::Write, path::PathBuf};
use console::style;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncWriteExt};
use crate::download::{api::{itch_api_game_info, itch_api_game_uploads, itch_api_upload_download, GameUpload}, downloader::download, utils::extract_archive};

use super::{config::Config, game::Game};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Library {
    pub games: Vec<Game>,
}



impl Library {

    async fn get_library_json_file(config: &Config) -> Result<PathBuf, Box<dyn Error>> {
        let games_path = PathBuf::from(&config.games_dir);
        fs::create_dir_all(&games_path).await?;

        let mut library_info_path = PathBuf::from(&games_path);
        library_info_path.push("library_info.json");

        Ok(library_info_path)
    }

    pub async fn load(config: &Config) -> Result<Library, Box<dyn Error>> {
        let path = Self::get_library_json_file(config).await?;
        let str = fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&str)?)
    }

    pub async fn save(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
        let str = serde_json::to_string_pretty(self)?;
        let path = Self::get_library_json_file(config).await?;
        let mut file = fs::File::create(path).await?;
        file.write(str.as_bytes()).await?;
        file.flush().await?;
        Ok(())
    }



    pub fn get_game(&mut self, _config: &Config, game_id: &i64) -> Option<&Game> {
        self.games.iter().find(|game| { game.game_id.eq(game_id) })
    }

    pub fn set_game(&mut self, config: &Config, game: &Game) {
        self.remove_game(config, game);
        self.games.push(game.clone());
    }

    pub fn remove_game(&mut self, _config: &Config, game: &Game) {
        match self.games.iter().enumerate().find(|(_, g)| { g.game_id.eq(&game.game_id) }) {
            Some((i, _)) => { self.games.remove(i); },
            _ => { },
        }
    }



    pub async fn download_game(&mut self, config: &Config, game_id: i64) -> Result<Option<&Game>, Box<dyn Error>> {
        println!("    {}", style("Getting game info").magenta());

        // Get game info.
        let game_info = itch_api_game_info(&config.api_key, &game_id).await?.game;
        if game_info.id != game_id {
            return Err("Game id does not match.".into());
        }

        // Get latest upload.
        let mut game_uploads = itch_api_game_uploads(&config.api_key, &game_info.id).await?.uploads;
        game_uploads.sort_by(|a, b| {
            a.id.cmp(&b.id)
        });
        let game_upload = game_uploads.iter()
            .filter(|game_upload| {
                if !game_upload.p_windows { return false; }
                match &game_upload.host {
                    Some(host) => {
                        host == "mega.nz" ||
                        host == "mega.co.nz"
                    },
                    None => true
                }
            })
            .collect::<Vec<&GameUpload>>().first().unwrap().to_owned();
        println!("    {} {}", style("File to download").magenta(), style(&game_upload.filename).magenta().bold());

        // Upload link to download.
        let game_download = itch_api_upload_download(&config.api_key, &game_upload.id).await?;

        // Download game.
        println!("    {}", style("Initializing download").magenta());
        let mut temp_path = PathBuf::from(&config.games_dir);
        temp_path.push("temp");
        temp_path.push(&game_upload.filename);

        let progress_bar = ProgressBar::new(0);
        progress_bar.set_style(ProgressStyle::with_template("    {msg:.magenta} {spinner:.cyan} [{elapsed_precise:.cyan}] [{bar:20.magenta/cyan}] {bytes:.cyan}/{total_bytes:.cyan} ({eta:.cyan})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));
        progress_bar.set_message("Downloading");

        download(game_download.url, &temp_path, |total_size, current_size| {
            progress_bar.set_position(current_size);
            progress_bar.set_length(total_size);
        }).await?;

        // Extract game archive.
        println!("    {}", style("Extracting game").magenta());
        let games_path = PathBuf::from(&config.games_dir);
        let mut game_path = PathBuf::from(&games_path);
        game_path.push(game_info.id.to_string());
        extract_archive(&temp_path, &game_path).await?;

        // Cleanup temp
        println!("    {}", style("Finishing installation").magenta());
        fs::remove_file(&temp_path).await?;

        // Add game to self
        self.set_game(config, &Game {
            game_id: game_info.id,
            upload_id: game_upload.id,
            title: game_info.title,
            description: game_info.short_text.unwrap_or("No description".into()),
            url: game_info.url,
            directory: game_path.strip_prefix(&games_path)?.to_str().unwrap().into(),
        });

        // Update library_info.json
        self.save(config).await?;

        Ok(self.get_game(config, &game_id))
    }

    // pub async fn delete_game(&mut self, game_id: i64) -> Result<(), Box<dyn Error>> {

    //     let game = self.get_game(&game_id);
    //     if game.is_none() {
    //         return Ok(());
    //     }
    //     let game = game.unwrap();

    //     println!("    {}", style("Deleting game").magenta());
    //     let mut game_path = PathBuf::from(&game.config.base_dir);
    //     game_path.push("games");
    //     game_path.push(&game.directory);

    //     fs::remove_dir_all(game_path).await?;

    //     self.remove_game(game);
    //     self.save().await?;

    //     Ok(())
    // }

}


