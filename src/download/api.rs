use std::error::Error;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct GameInfo {
    pub game: GameInfoGame,
}

#[derive(Deserialize)]
pub struct GameInfoGame {
    pub can_be_bought: bool,
    pub has_demo: bool,
    pub p_windows: bool,
    pub p_linux: bool,
    pub p_osx: bool,
    pub p_android: bool,
    pub min_price: i64,
    pub url: String,
    pub in_press_system: bool,
    pub user: GameUser,
    pub id: i64,
    pub r#type: String,
    pub cover_url: String,
    pub classification: String,
    pub created_at: String,
    pub published_at: String,
    pub title: String,
    pub short_text: Option<String>,
}

#[derive(Deserialize)]
pub struct GameEmbed {
    pub width: i64,
    pub height: i64,
    pub fullscreen: bool,
}

#[derive(Deserialize)]
pub struct GameUser {
    pub username: String,
    pub url: String,
    pub id: i64,
    pub cover_url: Option<String>,
}

pub async fn itch_api_game_info(api_key: &str, game_id: &i64) -> Result<GameInfo, Box<dyn Error>> {
    let url = format!("https://itch.io/api/1/{}/game/{}", api_key, game_id);
    Ok(reqwest::get(url).await?.json::<GameInfo>().await?)
}



#[derive(Deserialize)]
pub struct GameUploads {
    pub uploads: Vec<GameUpload>,
}

#[derive(Deserialize)]
pub struct GameUpload {
    pub id: i64,
    pub build_id: Option<i64>,
    pub position: i64,

    pub filename: String,
    pub size: Option<i64>,

    pub demo: bool,
    pub preorder: bool,

    pub storage: String,
    pub host: Option<String>,

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
pub struct GameUploadBuild {
    pub id: i64,
    pub parent_build_id: i64,
    pub version: i64,
    pub upload_id: i64,

    pub created_at: String,
    pub updated_at: String,

    pub user_version: Option<String>,
}

pub async fn itch_api_game_uploads(api_key: &str, game_id: &i64) -> Result<GameUploads, Box<dyn Error>> {
    let url = format!("https://itch.io/api/1/{}/game/{}/uploads", api_key, game_id);
    Ok(reqwest::get(url).await?.json::<GameUploads>().await?)
}



#[derive(Deserialize)]
pub struct UploadDownload {
    pub url: String,
}

pub async fn itch_api_upload_download(api_key: &str, upload_id: &i64) -> Result<UploadDownload, Box<dyn Error>> {
    let url = format!("https://itch.io/api/1/{}/upload/{}/download", api_key, upload_id);
    Ok(reqwest::get(url).await?.json::<UploadDownload>().await?)
}


