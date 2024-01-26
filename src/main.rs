
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
use clap::Parser;



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



fn display_help() {
    println!("{}", style("Invalid arguments").black().on_red());
    println!("For help use {}", style("itch-io-downloader.exe --help").bold());
}



/// Itch.io downloader
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to game library
    #[arg(index = 1)]
    library_path: String,
    /// The URL protocol program uses
    #[arg(index = 2)]
    url_protocol: String,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("{} {}", style("itch-io-downloader").magenta(), style(env!("CARGO_PKG_VERSION")).cyan());

    let base_dir = PathBuf::from(&args.library_path);

    let config = Config::load(base_dir).await?;

    // itch-io-downloader://
    match args.url_protocol.split("/").filter(|s| !s.is_empty()).collect::<Vec<&str>>()[..] {
        ["itch-io-downloader:", "play"] => {
            select_and_play(config).await?;
        },
        ["itch-io-downloader:", "play", game_id_str] => {
            let game_id: i64 = game_id_str.parse()?;
            download_and_execute(config, game_id).await?;
        },
        _ => {
            display_help();
        },
    };

    Ok(())
}


