#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GixDiscoveryError(#[from] gix::discover::Error),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
