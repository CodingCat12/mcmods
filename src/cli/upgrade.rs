use crate::lock::Version;
use crate::api;
use crate::io::{install, uninstall};
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use futures::future;
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Upgrade mods to their latest compatible versions.
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Project IDs to upgrade. Use `--all` to upgrade everything.
    #[arg(value_name = "PROJECT_ID")]
    project_ids: Vec<String>,

    /// Upgrade all installed mods.
    #[arg(long, short, conflicts_with = "project_ids")]
    all: bool,

    /// Mod loader to filter by (e.g., fabric, forge).
    #[arg(long, short, value_name = "LOADER")]
    loader: Option<api::Loader>,

    /// Minecraft version to filter by (e.g., 1.20.4).
    #[arg(long, short, value_name = "VERSION")]
    game_version: Option<String>,

    /// Release channel to filter by (e.g., release, beta, alpha).
    #[arg(long, short, value_name = "CHANNEL")]
    version_type: Option<api::VersionType>,

    /// Maximum number of concurrent tasks (default: 3).
    #[arg(long, value_name = "NUM")]
    max_concurrent_tasks: Option<usize>,
}

pub async fn cmd(
    lock: &mut Vec<Version>,
    args: Args,
    client: &Client,
    path: impl AsRef<Path>,
) -> Result<()> {
    let max_concurrent_tasks = args.max_concurrent_tasks.unwrap_or(3);
    let project_ids = if args.all {
        lock.iter().map(|v| v.project_id.clone()).collect()
    } else {
        args.project_ids.clone()
    };

    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = Vec::new();

    for project_id in project_ids {
        let semaphore = semaphore.clone();
        let client = client.clone();
        let loader = args.loader.clone();
        let game_version = args.game_version.clone();
        let version_type = args.version_type.clone();
        let lock = lock.clone();

        let task = async move {
            let _permit = semaphore.acquire().await?;

            let current_version = lock.iter().find(|v| v.project_id == project_id).unwrap();
            let version_ids = api::fetch_project(&client, &project_id).await?.versions;
            let available_versions = api::fetch_versions(&client, &version_ids).await?;

            let latest_version = available_versions.into_iter().rev().find(|v| {
                (if let Some(loader) = &loader {
                    v.loaders.contains(loader)
                } else {
                    current_version
                        .loaders
                        .iter()
                        .any(|l| v.loaders.contains(l))
                }) && (if let Some(game_version) = &game_version {
                    v.game_versions.contains(game_version)
                } else {
                    current_version
                        .game_versions
                        .iter()
                        .any(|gv| v.game_versions.contains(gv))
                }) && (if let Some(version_type) = &version_type {
                    v.version_type <= *version_type
                } else {
                    true
                })
            });

            latest_version
                .with_context(|| format!("No compatible version found for project {project_id}"))
                .map(|version| (project_id, version))
        };

        tasks.push(task);
    }

    let results = future::try_join_all(tasks).await?;

    for (project_id, version) in results {
        if let Some(existing_version) = lock.iter().find(|v| v.project_id == project_id) {
            if existing_version.id != version.id {
                let new_version = Version::from(version);
                if let Some(pos) = lock.iter().position(|v| v.project_id == project_id) {
                    lock[pos] = new_version;
                }
            }
        } else {
            let new_version = Version::from(version);
            lock.push(new_version);
        }
    }

    install(client, lock, &path, max_concurrent_tasks).await?;
    uninstall(lock, &path).await?;

    Ok(())
}
