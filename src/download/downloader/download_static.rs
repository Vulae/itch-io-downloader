use std::{error::Error, path::PathBuf};
use futures::StreamExt;
use reqwest::IntoUrl;
use tokio::{fs::{self, File}, io::AsyncWriteExt};



pub async fn download_static<U, F>(url: U, output: &PathBuf, on_progress: F) -> Result<(), Box<dyn Error>>
where
    U: IntoUrl,
    F: Fn(u64, u64)
{
    fs::create_dir_all(output.parent().unwrap()).await?;
    let mut file = File::create(output).await?;

    let request = reqwest::get(url).await?;
    let total_size = request.content_length().unwrap_or(0);
    let mut stream = request.bytes_stream();
    let mut current_size: u64 = 0;

    on_progress(total_size, current_size);

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        current_size += chunk.len() as u64;

        on_progress(total_size, current_size);
    }
    file.flush().await?;

    Ok(())
}


