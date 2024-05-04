use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tracing::debug;

pub mod constants;
pub mod error;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Project {
    /// The absolute path to the project root directory.
    /// This is the top-level directory of the project.
    pub root_directory: Option<PathBuf>,
    /// A unique identifier for the project.
    pub project_id: Option<String>,
    /// The directory for storing project specific configuration.
    pub config_home: Option<PathBuf>,
    /// The directory for storing project specific cache data.
    pub cache_home: Option<PathBuf>,
    /// The directory for storing project specific data files.
    pub data_home: Option<PathBuf>,
}

impl Project {
    /// Retrieve the project information detected from current directory.
    pub fn discover() -> Result<Self> {
        let project_root = get_project_root()?;
        let project_data = std::env::var(constants::PROJECT_DATA_HOME)
            .map(PathBuf::from)
            .ok();
        let project_config = std::env::var(constants::PROJECT_CONFIG_HOME)
            .map(PathBuf::from)
            .ok();
        let project_cache = std::env::var(constants::PROJECT_CACHE)
            .map(PathBuf::from)
            .ok();
        let project_id = std::env::var(constants::PROJECT_ID).ok();

        Ok(Self {
            root_directory: project_root,
            project_id,
            data_home: project_data,
            config_home: project_config,
            cache_home: project_cache,
        })
    }

    /// Retrieve the project information detected from the given directory.
    /// If a property is not set, then an opinionated default is used.
    pub async fn discover_and_assume() -> Result<Self> {
        let mut value = Self::discover()?;
        // If the project root is not found, give up.
        match value.root_directory {
            Some(_) => {}
            None => return Err(Error::ProjectRootNotFound(std::env::current_dir().unwrap())),
        }

        match value.config_home {
            Some(_) => {}
            None => {
                let mut directory = value.root_directory.clone().unwrap();
                directory.push(constants::DEFAULT_CONFIG_HOME);
                value.config_home = Some(directory);
            }
        }

        match value.data_home {
            Some(_) => {}
            None => {
                let mut directory = value.root_directory.clone().unwrap();
                directory.push(constants::DEFAULT_DATA_HOME);
                value.data_home = Some(directory);
            }
        }

        match value.cache_home {
            Some(_) => {}
            None => {
                let mut directory = value.root_directory.clone().unwrap();
                directory.push(constants::DEFAULT_CACHE_HOME);
                value.cache_home = Some(directory);
            }
        }

        match value.project_id {
            Some(_) => {}
            None => {
                let mut file = value.config_home.clone().unwrap();
                file.push(constants::PROJECT_ID_FILE);
                if file.exists() {
                    let mut file = tokio::fs::File::open(file).await.unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).await.unwrap();
                    value.project_id = Some(contents.trim().to_string());
                }
            }
        }

        Ok(value)
    }

    /// Retrieve the project information as a HashMap composed of the environment variables as keys
    /// and their values as values.
    pub fn project_hashmap(&self) -> std::collections::HashMap<String, Option<String>> {
        let mut hashmap = std::collections::HashMap::new();

        hashmap.insert(
            constants::PROJECT_ROOT.to_string(),
            self.root_directory
                .as_ref()
                .map(|p| p.to_str().unwrap().to_string()),
        );
        hashmap.insert(
            constants::PROJECT_DATA_HOME.to_string(),
            self.data_home
                .as_ref()
                .map(|p| p.to_str().unwrap().to_string()),
        );
        hashmap.insert(
            constants::PROJECT_CONFIG_HOME.to_string(),
            self.config_home
                .as_ref()
                .map(|p| p.to_str().unwrap().to_string()),
        );
        hashmap.insert(
            constants::PROJECT_CACHE.to_string(),
            self.cache_home
                .as_ref()
                .map(|p| p.to_str().unwrap().to_string()),
        );
        hashmap.insert(
            constants::PROJECT_ID.to_string(),
            self.project_id.as_ref().map(|p| p.to_string()),
        );

        hashmap
    }
}

/// An absolute path that points to the project root directory.
/// If the environment variable $PRJ_ROOT is set its value will be used.
/// Otherwise, a best effort is made to find the project root using the following technies:
/// - Searching upwards for a git repository
pub fn get_project_root() -> Result<Option<PathBuf>> {
    let project_root = std::env::var(constants::PROJECT_ROOT).ok();
    if let Some(project_root) = project_root {
        debug!(
            "using {} environment variable as project root",
            constants::PROJECT_ROOT
        );
        let path = PathBuf::from(project_root);
        return Ok(Some(path));
    }

    #[cfg(feature = "git")]
    {
        let current_dir = std::env::current_dir().unwrap();
        let git_repository = gix::discover(current_dir)?;
        if let Some(directory) = git_repository.work_dir() {
            debug!(?directory, "using git repository as project root");
            return Ok(Some(directory.to_owned()));
        }
    }

    Ok(None)
}
