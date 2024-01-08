
mod game;
mod library;
mod config;
mod api;

use std::{error::Error, path::PathBuf, io::Read};
use config::Config;
use library::Library;



fn console_question(question: &str) -> bool {
    println!("Y/N: {}", question);
    loop {
        let mut input = [0];
        let _ = std::io::stdin().read(&mut input);
        match input[0] as char {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => println!("\"Y\" or \"N\" only."),
        }
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Invalid arguments.");
        return Ok(());
    }

    let base_dir = PathBuf::from(&args[1]);
    // itch-io-downloader://GAME_ID/
    let game_id: i64 = args[2].split("/").collect::<Vec<&str>>()[2].parse()?;

    let config = Config::load(base_dir.clone()).await?;

    let mut library_path = PathBuf::from(&config.base_dir);
    library_path.push("games");
    let mut library = Library::new(config, library_path);
    library.create_dirs().await?;

    let game = match library.get_game(game_id).await? {
        // Game not installed, prompt to install.
        None => {
            if console_question("Game is not installed, do you want to install game?") {
                println!("Downloading game.");
                Some(library.download_game(game_id).await?)
            } else {
                None
            }
        },
        // Game installed, prompt to update if available.
        Some(mut game) => {
            println!("Game already installed.");
            if game.is_latest().await? {
                Some(game)
            } else {
                println!("Do you want to download the latest version of the game? Y/N");

                if console_question("Do you want to download the latest version of the game?") {
                    Some(library.download_game(game_id).await?)
                } else {
                    Some(game)
                }
            }
        },
    };

    // Start game
    match game {
        Some(mut game) => {
            println!("Starting game.");
            game.start().await?;
        },
        None => { }
    }

    Ok(())
}


