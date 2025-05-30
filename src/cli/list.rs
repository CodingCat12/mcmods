use crate::lock::Version;
use crate::api;
use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use std::path::Path;

/// List installed mods.
#[derive(Parser, Debug, Clone)]
pub struct Args;

pub async fn cmd(
    lock: &mut Vec<Version>,
    _args: Args,
    client: &Client,
    _path: impl AsRef<Path>,
) -> Result<()> {
    println!("Installed mods:");
    for version in lock {
        let project = api::fetch_project(client, &version.project_id).await?;
        let title = project.title;
        println!("{title}");
    }

    Ok(())
}
