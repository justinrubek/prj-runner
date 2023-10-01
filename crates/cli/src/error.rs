#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    PrjBaseDirectory(#[from] prj_base_directory::error::Error),
    #[error("command execution failed")]
    ExecError(i32),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;
