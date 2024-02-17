
use std::{path::PathBuf, error::Error};
use tokio::{fs, process::Command};

use super::error::DownloadError;



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
        let err_str = String::from_utf8_lossy(&result.stderr).to_string();
        Err(Box::new(DownloadError::ExtractFailed(err_str)))
    }
}


