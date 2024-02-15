use std::{error::Error, path::PathBuf, time::Duration};
use reqwest::IntoUrl;
use tokio::fs::{self, File};
use tokio_util::compat::TokioAsyncWriteCompatExt;
use async_read_progress::AsyncReadProgressExt;



pub async fn download_mega<U, F>(url: U, output: &PathBuf, on_progress: F) -> Result<(), Box<dyn Error>>
where
    U: IntoUrl,
    F: Fn(u64, u64)
{

    let url = url.into_url()?;
    if url.path_segments().unwrap().collect::<Vec<&str>>()[0] != "file" {
        panic!("Mega folder download not supported.");
    }

    let http_client = reqwest::Client::new();
    let mega = mega::Client::builder().build(http_client)?;

    let nodes = mega.fetch_public_nodes(url.as_str()).await?;
    let node = nodes.roots().filter(|node| node.kind().is_file()).collect::<Vec<_>>()[0];
    let total_size = node.size();

    fs::create_dir_all(output.parent().unwrap()).await?;
    let file = File::create(output).await?;

    let (reader, writer) = sluice::pipe::pipe();

    on_progress(total_size, 0);

    let reader = {
        reader.report_progress(Duration::from_millis(100), move |_current_size| {
            // on_progress(total_size, current_size as u64);
        })
    };

    let handle = tokio::spawn(async move { futures::io::copy(reader, &mut file.compat_write()).await });
    mega.download_node(node, writer).await?;
    handle.await.unwrap()?;

    println!("{}", nodes.len());
    
    Ok(())
}


