pub mod install;
pub mod remove;
pub mod upgrade;

pub mod list {
    use crate::LockVersion;
    use crate::api;
    use anyhow::Result;
    use clap::Parser;
    use reqwest::Client;
    use std::path::Path;

    #[derive(Parser, Debug, Clone)]
    pub struct Args;

    pub async fn cmd(
        lock: &mut Vec<LockVersion>,
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
}

pub mod sync {
    use crate::LockVersion;
    use crate::io::{install, uninstall};
    use anyhow::Result;
    use clap::Parser;
    use reqwest::Client;
    use std::path::Path;

    #[derive(Parser, Debug, Clone)]
    pub struct Args {
        #[arg(long)]
        max_concurrent_tasks: Option<usize>,
    }

    pub async fn cmd(
        lock: &mut [LockVersion],
        args: Args,
        client: &Client,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let max_concurrent_tasks = args.max_concurrent_tasks.unwrap_or(3);
        install(client, lock, &path, max_concurrent_tasks).await?;
        uninstall(lock, &path).await?;
        Ok(())
    }
}
