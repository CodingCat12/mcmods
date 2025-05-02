use anyhow::Result;
use digest::Digest;
use hex::encode;
use sha2::Sha512;
use std::path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn hash_file(path: &path::Path) -> Result<String> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha512::new();
    let mut buf = [0u8; 4096];
    loop {
        let bytes_read = file.read(&mut buf).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buf[..bytes_read]);
    }
    let data = hasher.finalize();
    Ok(encode(data))
}

pub async fn retain_async<T>(vec: &mut Vec<T>, f: impl AsyncFn(&T) -> bool) {
    let mut new_vec = Vec::with_capacity(vec.len());
    for item in vec.drain(..) {
        if f(&item).await {
            new_vec.push(item);
        }
    }
    *vec = new_vec;
}
