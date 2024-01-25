
use std::{path::PathBuf, error::Error};
use futures::StreamExt;
use reqwest::IntoUrl;
use tokio::{fs::{self, File}, io::AsyncWriteExt, process::Command};



pub async fn extract_archive(archive: &PathBuf, out_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(out_dir).await?;

    let result = Command::new("tar")
        .arg("-x")
        .arg("-f").arg(archive)
        .arg("-C").arg(out_dir)
        .output().await?;

    if result.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).into())
    }
}



pub async fn download_file<U: IntoUrl, F: Fn(u64, u64)>(url: U, out: &PathBuf, progress: F) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(out.parent().unwrap()).await?;
    let mut file = File::create(out).await?;

    let request = reqwest::get(url).await?;
    let total_size = request.content_length().unwrap_or(0);
    let mut stream = request.bytes_stream();
    let mut current_size: u64 = 0;

    progress(total_size, current_size);

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        current_size += chunk.len() as u64;

        progress(total_size, current_size);
    }
    file.flush().await?;

    Ok(())
}


