#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GixDiscoveryError(#[from] gix::discover::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
