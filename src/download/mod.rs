
use std::error::Error;
use console::style;
use dialoguer::{Confirm, Select};
use self::{config::Config, library::Library};

pub mod config;
mod utils;
mod game;
mod library;
mod api;
mod downloader;



pub async fn download_and_execute(config: &Config, game_id: i64) -> Result<(), Box<dyn Error>> {
    let mut library = Library::load(config).await?;

    // Get game and prompt of install if not already.
    let mut game = if let Some(game) = library.get_game(config, &game_id) {
        game
    } else {
        println!("{}", style("Game is not installed, do you want to install game?").magenta());

        let confirmation = Confirm::new()
            .report(false)
            .interact()?;

        if confirmation {
            println!("{}", style("Downloading game").magenta());
            library.download_game(config, game_id).await?;
            library.get_game(config, &game_id).unwrap()
        } else {
            return Ok(());
        }
    };

    // Check if game is up to date.
    if !(game.clone().is_latest(config).await?) {
        println!("{}", style("Do you want to download the latest version of the game?").magenta());

        let confirmation = Confirm::new()
            .report(false)
            .interact()?;

        if confirmation {
            library.download_game(config, game_id).await?;
        }
        
        game = library.get_game(config, &game_id).unwrap();
    };

    // Start game.
    println!("{}", style("Starting game").magenta());
    game.clone().start(config).await?;

    Ok(())
}



pub async fn select_and_play(config: &Config) -> Result<(), Box<dyn Error>> {
    let library = Library::load(config).await?;

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

    download_and_execute(config, library.games[selection - 1].game_id).await?;

    Ok(())
}


