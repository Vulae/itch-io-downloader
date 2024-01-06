
// TODO: Make paths more consistent using PathBuf instead of String.
// TODO: Some general cleanup on the code, specifically the downloading section.
// TODO: Use more tokio instead of normal things.
// TODO: Clean up running game executable.

use std::{error::Error, path::PathBuf, fs, process::Command};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt};
use futures::StreamExt;
use zip::ZipArchive;



fn api_url_game_uploads(api_key: &str, id: &i64) -> String {
    format!("https://itch.io/api/1/{}/game/{}/uploads", api_key, id)
}

fn api_url_upload_download(api_key: &str, id: &i64) -> String {
    format!("https://itch.io/api/1/{}/upload/{}/download", api_key, id)
}



#[derive(Deserialize)]
#[allow(dead_code)]
struct GameUploads {
    pub uploads: Vec<GameUpload>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct GameUpload {
    pub id: i64,
    pub build_id: Option<i64>,
    pub position: i64,

    pub filename: String,
    pub size: i64,

    pub demo: bool,
    pub preorder: bool,

    pub storage: String,

    pub created_at: String,
    pub updated_at: String,

    pub p_windows: bool,
    pub p_osx: bool,
    pub p_linux: bool,
    pub p_android: bool,

    pub build: Option<GameUploadBuild>,
    pub channel_name: Option<String>,
    pub r#type: String,
    pub game_id: i64,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct GameUploadBuild {
    pub id: i64,
    pub parent_build_id: i64,
    pub version: i64,
    pub upload_id: i64,

    pub created_at: String,
    pub updated_at: String,

    pub user_version: String,
}



#[derive(Deserialize)]
#[allow(dead_code)]
struct UploadDownload {
    url: String,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Invalid arguments.");
        return Ok(());
    }

    let base_dir = &args[1];
    // itch-io-downloader://GAME_ID/
    let id: i64 = args[2].split("/").collect::<Vec<&str>>()[2].parse()?;

    println!("Base path \"{}\"", base_dir);
    println!("Game {}", id);



    let api_key_path = format!("{}/api_key.txt", base_dir);
    let dir_path = format!("{}/games/{}", base_dir, id);
    let temp_path = format!("{}/games/temp", base_dir);



    // Download game if not already.
    let dir_exists = fs::metadata(&dir_path);
    if !dir_exists.is_ok() || !dir_exists?.is_dir() {
        
        // API Key
        println!("Reading API key.");
        let api_key = fs::read_to_string(api_key_path)?;
        
        // Get download URL.
        println!("Getting game download.");
        let game_uploads_req = reqwest::get(api_url_game_uploads(&api_key, &id)).await?;
        let mut game_uploads =  game_uploads_req.json::<GameUploads>().await?.uploads;
        game_uploads.sort_by(|a, b| {
            a.id.cmp(&b.id)
        });
        let game_upload = game_uploads.iter()
            .filter(|game_upload| game_upload.p_windows)
            .collect::<Vec<&GameUpload>>().first().unwrap().to_owned();

        let game_download_req = reqwest::get(api_url_upload_download(&api_key, &game_upload.id)).await?;
        let game_download = game_download_req.json::<UploadDownload>().await?.url;

        // Download ZIP file.
        fs::create_dir_all(&temp_path)?;
        let temp_file = format!("{}/{}.zip", temp_path, id);
        let mut file = File::create(&temp_file).await?;
        println!("Downloading game, this may take a while. . .");
        let mut stream = reqwest::get(game_download).await?.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            file.write_all(&chunk).await?;
        }
        file.flush().await?;

        // Unzip file.
        fs::create_dir_all(&dir_path)?;
        println!("Extracting game, this may take a while. . .");
        let zip_file = std::fs::File::open(&temp_file)?;
        let mut archive = ZipArchive::new(zip_file)?;
        archive.extract(&dir_path)?;

        // Clean up temp files.
        println!("Cleaning up.");
        fs::remove_file(&temp_file)?;

        println!("Successfully downloaded game.");
    } else {
        println!("Game already downloaded.");
    }



    // Find most likely to be game start executable and run it.
    if !fs::metadata(&dir_path)?.is_dir() {
        return Err(format!("\"{}\" is not dir?", dir_path).into());
    }

    println!("Starting game {}.", id);

    fn search_exe(path: PathBuf) -> Result<Option<PathBuf>, Box<dyn Error>> {
        // TODO: Currently we just return the first executable.
        // This may break with games that include multiple executables. (eg: UnityCrashHandler64.exe).
        // We will want to blacklist those and only search for the biggest executable.
        let mut queue: Vec<PathBuf> = vec![ path ];

        while queue.len() > 0 {
            let path = queue.remove(0);

            if path.is_dir() {
                for entry in path.read_dir()? {
                    queue.push(entry?.path());
                }
            } else if path.is_file() {
                if path.extension().is_some() && path.extension().unwrap() == "exe" {
                    return Ok(Some(path.to_owned()));
                }
            }
        }

        Ok(None)
    }

    let executable_path = search_exe(PathBuf::from(dir_path))?;

    if executable_path.is_some() {
        let full_path = fs::canonicalize(executable_path.unwrap())?;
        // I'm dumb and don't know how canonicalize works, it adds \\?\ to beginning?
        let (_, exec_path) = full_path.to_str().unwrap().split_at(4);
        println!("Executing game: \"{}\".", exec_path);
        let _program = Command::new(exec_path).spawn().unwrap();
    } else {
        println!("Failed to find executable.");
    }



    Ok(())
}


