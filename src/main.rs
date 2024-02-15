
mod download;
use std::error::Error;
use clap::{Parser, Subcommand};
use download::{config::Config, download_and_execute, select_and_play};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Select a game to play
    SelectPlay { },
    /// Play a game
    Play {
        #[arg(index = 1)]
        game_id: Option<i64>,
    },
    #[command(hide = true)]
    Uri {
        #[arg(index = 1)]
        uri: String,
    },
}



async fn get_config() -> Result<Config, Box<dyn Error>> {
    // TODO: Clean up.
    let mut base_dir = std::env::current_exe()?;
    base_dir = base_dir.parent().unwrap().to_path_buf();
    if cfg!(debug_assertions) {
        base_dir = base_dir.parent().unwrap().parent().unwrap().to_path_buf();
    }
    Ok(Config::load(base_dir).await?)
}

async fn play(game_id: i64) -> Result<(), Box<dyn Error>> {
    download_and_execute(get_config().await?, game_id).await?;
    Ok(())
}

async fn select_play() -> Result<(), Box<dyn Error>> {
    select_and_play(get_config().await?).await?;
    Ok(())
}





#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Play { game_id } => {
            match game_id {
                Some(game_id) => play(game_id.clone()).await?,
                None => select_play().await?,
            }
            Ok(())
        },
        Commands::Uri { uri } => {
            match uri.split("/").filter(|s| !s.is_empty()).collect::<Vec<&str>>()[..] {
                ["itch-io-downloader:", "play", game_id_str] => {
                    let game_id: i64 = game_id_str.parse()?;
                    play(game_id).await?;
                },
                ["itch-io-downloader:", "play"] => {
                    select_play().await?;
                },
                _ => {
                    panic!("Invalid URI.");
                },
            }
            Ok(())
        },
        _ => panic!("Invalid arguments."),
    }
}


