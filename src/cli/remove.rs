use crate::io::uninstall;
use std::path::Path;

use crate::lock::Version;
use crate::api;
use crate::util::retain_async;
use anyhow::Result;
use clap::Parser;
use reqwest::Client;

/// Remove a mod by project ID or slug.
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// ID or slug of the project to remove.
    #[arg(value_name = "PROJECT_ID")]
    project_id: String,
}

pub async fn cmd(
    lock: &mut Vec<Version>,
    args: Args,
    client: &Client,
    path: impl AsRef<Path>,
) -> Result<()> {
    let project_id = args.project_id.clone();
    retain_async(lock, async |v: &Version| {
        let p = api::fetch_project(client, &v.project_id).await.unwrap();
        p.id != project_id && p.slug != Some(project_id.clone())
    })
    .await;

    uninstall(lock, path).await?;

    Ok(())
}
