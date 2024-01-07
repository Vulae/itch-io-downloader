
mod download;
mod run;

use std::{error::Error, path::PathBuf, fs};

use crate::{download::download_game, run::run_game};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Invalid arguments.");
        return Ok(());
    }

    let base_dir = PathBuf::from(&args[1]);
    // itch-io-downloader://GAME_ID/
    let id: i64 = args[2].split("/").collect::<Vec<&str>>()[2].parse()?;

    println!("Base path \"{}\"", base_dir.to_str().unwrap());
    println!("Game {}", id);



    let mut games_dir = PathBuf::from(&base_dir);
    games_dir.push("games");



    // Download game if not already.
    if {
        let mut game_dir = PathBuf::from(&games_dir);
        game_dir.push(id.to_string());
        !game_dir.exists() && !game_dir.is_dir()
    } {
        
        println!("Reading API key.");
        let mut api_key_path = PathBuf::from(&base_dir);
        api_key_path.push("api_key.txt");
        let api_key = fs::read_to_string(api_key_path)?;
        
        download_game(&api_key, &id, &games_dir).await?;

    } else {
        println!("Game already downloaded.");
    }



    run_game(&id, &games_dir).await?;



    Ok(())
}


