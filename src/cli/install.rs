use crate::LockVersion;
use crate::api;
use crate::io::install;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use futures::future;
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Install the latest compatible versions of specified mods.
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Project IDs of the mods to install.
    #[arg(value_name = "PROJECT_ID", required = true)]
    project_ids: Vec<String>,

    /// Modloader to filter by (e.g., fabric, forge).
    #[arg(
        long,
        short,
        value_name = "LOADER",
        help = "Optional loader type to match against."
    )]
    loader: Option<api::Loader>,

    /// The game version to filter versions by (e.g., 1.21.4).
    #[arg(
        long,
        short,
        value_name = "VERSION",
        help = "Optional Minecraft version to match."
    )]
    game_version: Option<String>,

    /// The release channel to use (e.g., release, beta, alpha).
    #[arg(
        long,
        short,
        value_name = "CHANNEL",
        help = "Version channel to filter (default: release)."
    )]
    version_type: Option<api::VersionType>,

    /// Maximum number of concurrent download tasks (default: 3).
    #[arg(
        long,
        value_name = "NUM",
        help = "Set maximum concurrent tasks (default: 3)."
    )]
    max_concurrent_tasks: Option<usize>,
}

pub async fn cmd(
    lock: &mut Vec<LockVersion>,
    args: Args,
    client: &Client,
    path: impl AsRef<Path>,
) -> Result<()> {
    let max_concurrent_tasks = args.max_concurrent_tasks.unwrap_or(3);
    let project_ids = args.project_ids;
    let loader = args.loader;
    let game_version = args.game_version;
    let version_type = args.version_type;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = Vec::new();

    for p in project_ids {
        let semaphore = semaphore.clone();
        let client = client.clone();
        let loader = loader.clone();
        let game_version = game_version.clone();
        let version_type = version_type.clone().unwrap_or(api::VersionType::Release);
        let task = async move {
            let _permit = semaphore
                .acquire()
                .await
                .with_context(|| format!("Acquiring semaphore for project '{p}'"))?;
            let version_ids = api::fetch_project(&client, &p)
                .await
                .with_context(|| format!("Failed to fetch project metadata for '{p}'"))?
                .versions;
            let available_versions = api::fetch_versions(
                &client,
                &version_ids[(version_ids.len() as i32 - 500).max(0) as usize..],
            )
            .await
            .with_context(|| format!("Failed to fetch versions for project '{p}'"))?;

            let latest_version = available_versions.into_iter().rev().find(|v| {
                (if let Some(loader) = &loader {
                    v.loaders.contains(loader)
                } else {
                    true
                }) && (if let Some(game_version) = &game_version {
                    v.game_versions.contains(game_version)
                } else {
                    true
                }) && v.version_type <= version_type
            });

            latest_version.with_context(|| format!("No matching version found for '{p}'"))
        };
        tasks.push(task);
    }

    let versions = future::try_join_all(tasks).await?;

    for version in versions {
        let version = LockVersion::from(version);
        lock.push(version);
    }

    install(client, lock, path, max_concurrent_tasks).await?;

    Ok::<_, anyhow::Error>(())
}
