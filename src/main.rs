
mod game;
mod library;
mod config;
mod api;
mod utils;

use std::{error::Error, path::PathBuf};
use config::Config;
use dialoguer::{Select, Confirm};
use library::Library;
use console::style;



async fn get_library(config: Config) -> Result<Library, Box<dyn Error>> {
    let mut library_path = PathBuf::from(&config.base_dir);
    library_path.push("games");
    Ok(Library::load(config).await?)
}



async fn download_and_execute(config: Config, game_id: i64) -> Result<(), Box<dyn Error>> {
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



async fn select_and_play(config: Config) -> Result<(), Box<dyn Error>> {
    let library = get_library(config.clone()).await?;

    if library.games.len() == 0 {
        println!("{}", style("Library has no games").black().on_red());
        return Ok(());
    }

    println!("{}", style("Select a installed game to play:").magenta());

    let selection = Select::new()
        .report(false)
        .items(&(library.games.iter().map(|game| {
            format!("{}", style(game.title.clone()).magenta())
        }).collect::<Vec<String>>()))
        .default(0)
        .interact()?;

    download_and_execute(config.clone(), library.games[selection].game_id).await?;

    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    // itch-io-downloader.exe "path/to/install/location" "URL PROTOCOL"

    if args.len() != 3 {
        println!("{}", style("Invalid arguments.").black().on_red());
        return Ok(());
    }

    println!();
    println!("{} {}", style("itch-io-downloader").magenta(), style(env!("CARGO_PKG_VERSION")).cyan());

    let base_dir = PathBuf::from(&args[1]);

    let config = Config::load(base_dir.clone()).await?;

    // itch-io-downloader://
    let url_protocol = &args[2];

    // itch-io-downloader://
    match url_protocol.split("/").filter(|s| !s.is_empty()).collect::<Vec<&str>>()[..] {
        ["itch-io-downloader:", "play"] => {
            select_and_play(config).await?;
            ()
        },
        ["itch-io-downloader:", "play", game_id_str] => {
            let game_id: i64 = game_id_str.parse()?;
            download_and_execute(config, game_id).await?;
            ()
        },
        [_] => { },
        _ => { },
    }

    println!();

    Ok(())
}


