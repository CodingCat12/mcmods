use crate::api;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Version {
    pub loaders: Vec<api::Loader>,
    pub game_versions: Vec<String>,
    pub id: String,
    pub project_id: String,
    pub files: Vec<VersionFile>,
}

impl From<api::Version> for Version {
    fn from(value: api::Version) -> Self {
        Self {
            loaders: value.loaders,
            game_versions: value.game_versions,
            id: value.id,
            project_id: value.project_id,
            files: value.files.into_iter().map(|f| f.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct VersionFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

impl From<api::VersionFile> for VersionFile {
    fn from(value: api::VersionFile) -> Self {
        Self {
            hashes: value.hashes.into(),
            url: value.url,
            filename: value.filename,
            primary: value.primary,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Hashes {
    pub sha512: String,
}

impl From<api::Hashes> for Hashes {
    fn from(value: api::Hashes) -> Self {
        Self {
            sha512: value.sha512,
        }
    }
}
