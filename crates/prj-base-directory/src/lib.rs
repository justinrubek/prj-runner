use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tracing::debug;

pub mod error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub project_root: Option<PathBuf>,
    pub project_id: Option<String>,
}

impl Project {
    /// Retrieve the project information detected from current directory.
    pub async fn discover() -> Result<Self> {
        let project_root = get_project_root().await?;
        let project_id = get_project_id().await;

        Ok(Self {
            project_root,
            project_id,
        })
    }
}

/// An absolute path that points to the project root directory.
/// If the environment variable $PRJ_ROOT is set its value will be used.
/// Otherwise, a best effort is made to find the project root using the following technies:
/// - Searching upwards for a git repository
#[tracing::instrument]
pub async fn get_project_root() -> Result<Option<PathBuf>> {
    let project_root = std::env::var("PRJ_ROOT").ok();
    if let Some(project_root) = project_root {
        debug!("Using PRJ_ROOT environment variable as project root");
        let path = PathBuf::from(project_root);
        return Ok(Some(path));
    }

    #[cfg(feature = "git")]
    {
        let current_dir = std::env::current_dir().unwrap();
        let git_repository = gix::discover(current_dir)?;
        if let Some(directory) = git_repository.work_dir() {
            debug!(?directory, "Using git repository as project root");
            return Ok(Some(directory.to_owned()));
        }
    }

    Ok(None)
}

/// The project id is an optional unique identifier for a project.
/// Specification
///
/// The PRJ_ID value MUST pass the following regular expression: ^[a-zA-Z0-9_-]{,32}$. It can be a UUIDv4 or some other random identifier.
/// If the environment variable $PRJ_ID is set, it MUST be used as the project id.
/// Otherwise, if the PRJ_CONFIG_HOME is set and a prj_id file exists, it will be loaded after stripping any trailing white spaces.
/// Otherwise, the tool is free to pick its own logic.
pub async fn get_project_id() -> Option<String> {
    let project_id = std::env::var("PRJ_ID").ok();
    if project_id.is_some() {
        return project_id;
    }

    let config_home = std::env::var("PRJ_CONFIG_HOME").ok();
    if config_home.is_some() {
        let mut path = std::path::PathBuf::from(config_home.unwrap());
        path.push("prj_id");
        if path.exists() {
            let mut file = tokio::fs::File::open(path).await.unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).await.unwrap();
            return Some(contents.trim().to_string());
        }
    }

    None
}
