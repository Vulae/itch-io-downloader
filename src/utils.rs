
use std::{path::PathBuf, error::Error, io::Read};
use tokio::{process::Command, fs};



pub fn console_question(question: &str) -> bool {
    println!("Y/N: {}", question);
    loop {
        let mut input = [0];
        let _ = std::io::stdin().read(&mut input);
        match input[0] as char {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => println!("\"Y\" or \"N\" only."),
        }
    }
}



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


