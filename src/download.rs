
use std::{error::Error, path::PathBuf};
use futures::StreamExt;
use serde::Deserialize;
use tokio::{fs::{File, self}, io::AsyncWriteExt};
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



pub async fn download_game(api_key: &str, id: &i64, games_dir: &PathBuf) -> Result<(), Box<dyn Error>> {

    let mut game_dir = PathBuf::from(games_dir);
    game_dir.push(id.to_string());

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
    let mut temp_path = PathBuf::from(games_dir);
    temp_path.push(format!("temp/{}.zip", id));
    let mut file = File::create(&temp_path).await?;
    println!("Downloading game, this may take a while. . .");
    let mut stream = reqwest::get(game_download).await?.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    // Unzip file.
    fs::create_dir_all(&game_dir).await?;
    println!("Extracting game, this may take a while. . .");
    let zip_file = std::fs::File::open(&temp_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    archive.extract(&game_dir)?;

    // Clean up temp files.
    println!("Cleaning up.");
    fs::remove_file(&temp_path).await?;

    Ok(())

}


