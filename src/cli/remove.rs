use crate::io::uninstall;
use std::path::Path;

use crate::LockVersion;
use crate::api;
use crate::util::retain_async;
use anyhow::Result;
use clap::Parser;
use reqwest::Client;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    project_id: String,
}

pub async fn cmd(
    lock: &mut Vec<LockVersion>,
    args: Args,
    client: &Client,
    path: impl AsRef<Path>,
) -> Result<()> {
    let project_id = args.project_id.clone();
    retain_async(lock, async |v: &LockVersion| {
        let p = api::fetch_project(client, &v.project_id).await.unwrap();
        p.id != project_id && p.slug != Some(project_id.clone())
    })
    .await;

    uninstall(lock, path).await?;

    Ok(())
}
