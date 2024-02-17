
mod download;
use std::error::Error;
use clap::{Parser, Subcommand};
use download::{config::Config, download_and_execute, select_and_play};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
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



async fn play(game_id: i64) -> Result<(), Box<dyn Error>> {
    download_and_execute(&Config::load().await?, game_id).await?;
    Ok(())
}

async fn select_play() -> Result<(), Box<dyn Error>> {
    select_and_play(&Config::load().await?).await?;
    Ok(())
}





#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        None => select_play().await?,

        Some(Commands::Play { game_id }) => {
            match game_id {
                Some(game_id) => play(game_id.clone()).await?,
                None => select_play().await?,
            }
        },

        Some(Commands::Uri { uri }) => {
            match uri.split("/").filter(|s| !s.is_empty()).collect::<Vec<&str>>()[..] {
                ["itch-io-downloader:", "play", id] => play(id.parse()?).await?,
                ["itch-io-downloader:", "play"] => select_play().await?,
                _ => panic!("Invalid URI."),
            }
        }
    };

    Ok(())
}


