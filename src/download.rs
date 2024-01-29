use std::{error::Error, path::PathBuf};
use reqwest::IntoUrl;

use self::{download_mega::download_mega, download_static::download_static};

mod download_static;
mod download_mega;



pub async fn download<U, F>(url: U, output: &PathBuf, on_progress: F) -> Result<(), Box<dyn Error>>
where
    U: IntoUrl,
    F: Fn(u64, u64)
{

    let url = url.into_url()?;

    // TODO: Better error handling (I'm too lazy. . .)

    match url.domain() {
        Some("mega.nz") | Some("mega.co.nz") => {
            panic!("Mega download W.I.P.");
            // download_mega(url, output, on_progress).await?;
        },
        // Assume everything else is just static.
        Some(_) => {
            download_static(url, output, on_progress).await?;
        },
        _ => { },
    }

    Ok(())
}


