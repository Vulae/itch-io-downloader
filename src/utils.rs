
use std::{path::PathBuf, error::Error};
use tokio::{process::Command, fs};



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


