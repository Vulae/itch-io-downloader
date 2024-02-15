
use std::{error::Error, path::PathBuf};
use console::style;
use dialoguer::{Confirm, Select};

use self::{config::Config, library::Library};

pub mod config;
mod utils;
mod game;
mod library;
mod api;
mod downloader;



async fn get_library(config: Config) -> Result<Library, Box<dyn Error>> {
    let mut library_path = PathBuf::from(&config.base_dir);
    library_path.push("games");
    Ok(Library::load(config).await?)
}



pub async fn download_and_execute(config: Config, game_id: i64) -> Result<(), Box<dyn Error>> {
    let mut library = get_library(config).await?;

    let game = match library.get_game(&game_id) {
        // Game not installed, prompt to install.
        None => {
            println!("{}", style("Game is not installed, do you want to install game?").magenta());

            let confirmation = Confirm::new()
                .report(false)
                .interact()?;

            if confirmation {
                println!("{}", style("Downloading game").magenta());
                library.download_game(game_id).await?;
                library.get_game(&game_id)
            } else {
                None
            }
        },
        // Game installed, prompt to update if available.
        Some(game) => {
            println!("{}", style("Game already installed").magenta());
            if game.clone().is_latest().await? {
                Some(game)
            } else {
                println!("{}", style("Do you want to download the latest version of the game?").magenta());

                let confirmation = Confirm::new()
                    .report(false)
                    .interact()?;

                if confirmation {
                    library.download_game(game_id).await?;
                }
                
                library.get_game(&game_id)
            }
        },
    };

    match game {
        Some(game) => {
            println!("{}", style("Starting game").magenta());
            game.clone().start().await?;
        },
        None => { }
    }

    Ok(())
}



pub async fn select_and_play(config: Config) -> Result<(), Box<dyn Error>> {
    let library = get_library(config.clone()).await?;

    if library.games.len() == 0 {
        println!("{}", style("Library has no games").black().on_red());
        return Ok(());
    }

    println!("{}", style("Select a installed game to play:").magenta());

    let selection = Select::new()
        .report(false)
        .item(format!("{}", style("none").magenta()))
        .items(&(library.games.iter().map(|game| {
            format!("{}", style(game.title.clone()).magenta().bright())
        }).collect::<Vec<String>>()))
        .default(0)
        .interact()?;

    if selection == 0 {
        println!("{}", style("No game selected").magenta());
        return Ok(());
    }

    download_and_execute(config.clone(), library.games[selection - 1].game_id).await?;

    Ok(())
}


