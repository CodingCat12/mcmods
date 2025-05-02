use mcmods::cli;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use reqwest::Client;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
    #[arg(long, short)]
    verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    Install(cli::install::Args),
    Remove(cli::remove::Args),
    Sync(cli::sync::Args),
    Upgrade(cli::upgrade::Args),
    List(cli::list::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let mods_path = "mods";
    if !fs::try_exists(mods_path).await? {
        fs::create_dir(mods_path).await?;
    }

    let lock_path = "mods/lock.json";

    let mut lock = if !fs::try_exists(lock_path).await? {
        let mut f = fs::File::create(lock_path).await?;
        f.write_all("[]".as_bytes()).await?;
        Vec::new()
    } else {
        let contents = tokio::fs::read_to_string(lock_path).await?;
        serde_json::from_str(&contents)?
    };

    let args = Args::parse();

    match args.command {
        Command::Install(args) => cli::install::cmd(&mut lock, args, &client, mods_path).await,
        Command::Remove(args) => cli::remove::cmd(&mut lock, args, &client, mods_path).await,
        Command::Sync(args) => cli::sync::cmd(&mut lock, args, &client, mods_path).await,
        Command::List(args) => cli::list::cmd(&mut lock, args, &client, mods_path).await,
        Command::Upgrade(args) => cli::upgrade::cmd(&mut lock, args, &client, mods_path).await,
    }?;

    let json = serde_json::to_string_pretty(&lock)?;
    fs::write(lock_path, json).await?;

    Ok(())
}
