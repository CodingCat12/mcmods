use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn fetch_project(client: &Client, id: &str) -> Result<Project> {
    let url = format!("https://api.modrinth.com/v2/project/{id}");
    Ok(client.get(url).send().await?.json().await?)
}

pub async fn fetch_version(client: &Client, id: &str) -> Result<Version> {
    let url = format!("https://api.modrinth.com/v2/version/{id}");
    Ok(client.get(url).send().await?.json().await?)
}

pub async fn fetch_versions(client: &Client, ids: &[String]) -> Result<Vec<Version>> {
    let url = format!("https://api.modrinth.com/v2/versions?ids={ids:?}");
    Ok(client.get(url).send().await?.json().await?)
}

pub async fn fetch_version_files(client: &Client, hash: &str) -> Result<Version> {
    let url = format!("https://api.modrinth.com/v2/version_file/{hash}");
    Ok(client.get(url).send().await?.json().await?)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub versions: Vec<String>,
    pub id: String,
    pub slug: Option<String>,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Version {
    pub game_versions: Vec<String>,
    pub loaders: Vec<Loader>,
    pub id: String,
    pub project_id: String,
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub changelog_url: Option<String>,
    pub downloads: u32,
    pub files: Vec<VersionFile>,
    pub dependencies: Vec<Dependency>,
    pub version_type: VersionType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Loader {
    Fabric,
    Neoforge,
    Quilt,
    Forge,
    ModLoader, // Risugami's Modloader
    LiteLoader,
    Rift,
    Minecraft, // for resource packs
    Datapack,
    Folia,
    Paper,
    Purpur,
    Bukkit,
    Spigot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

impl std::cmp::Ord for VersionType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match *self {
            Self::Release => 1,
            Self::Beta => 2,
            Self::Alpha => 3,
        }
        .cmp(&match *other {
            Self::Release => 1,
            Self::Beta => 2,
            Self::Alpha => 3,
        })
    }
}

impl std::cmp::PartialOrd for VersionType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct VersionFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}
