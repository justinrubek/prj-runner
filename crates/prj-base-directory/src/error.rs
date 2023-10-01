#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GixDiscoveryError(#[from] gix::discover::Error),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    #[error(transparent)]
    StdEnv(#[from] std::env::VarError),

    #[error("failed to find project root directory in search from {0}")]
    ProjectRootNotFound(std::path::PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;
