use crate::lock::Version;
use crate::util::hash_file;
use anyhow::Result;
use futures::StreamExt;
use futures::future;
use reqwest::Client;
use std::path;
use std::sync::Arc;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;

pub async fn install(
    client: &Client,
    lock: &[Version],
    path: impl AsRef<path::Path>,
    max_concurrent_tasks: usize,
) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = Vec::new();

    for version in lock {
        if let Some(file) = version.files.iter().find(|f| f.primary) {
            let url = file.url.clone();
            let filename = file.filename.clone();
            let client = client.clone();
            let path = path.as_ref();
            let semaphore = semaphore.clone();

            let task = async move {
                let _permit = semaphore.acquire().await?;
                let file_path = path.join(&filename);
                if !fs::try_exists(&file_path).await? {
                    let resp = client.get(&url).send().await?;
                    let mut stream = resp.bytes_stream();

                    let mut dest = File::create(&file_path).await?;
                    while let Some(chunk) = stream.next().await {
                        let bytes = chunk?;
                        dest.write_all(&bytes).await?;
                    }
                }
                Ok::<_, anyhow::Error>(())
            };

            tasks.push(task);
        }
    }

    future::try_join_all(tasks).await?;
    Ok(())
}

pub async fn uninstall(lock: &[Version], path: impl AsRef<path::Path>) -> Result<()> {
    let mut entries = fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let file_path = entry.path();

        if file_path.is_dir() || file_path.extension().and_then(|e| e.to_str()) != Some("jar") {
            continue;
        }

        let hash = hash_file(&file_path).await?;

        let is_in_lock = lock.iter().any(|v| {
            v.files
                .iter()
                .find(|v| v.primary)
                .map(|f| f.hashes.sha512 == hash)
                .unwrap_or(false)
        });

        if !is_in_lock {
            fs::remove_file(&file_path).await?;
        }
    }

    Ok(())
}
