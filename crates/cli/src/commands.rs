use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Project(Project),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Project {
    #[clap(subcommand)]
    pub command: ProjectCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum ProjectCommands {
    /// Run a command in the project's root directory
    Exec(Exec),
    /// Display the project structure information
    Info(Info),
    /// Generate environment variables with the project's structure
    GenerateEnv(Env),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Exec {
    /// The command to run
    #[arg()]
    pub command: String,
    /// The arguments to pass to the command
    #[arg()]
    pub args: Vec<String>,
}

#[derive(clap::Args, Debug)]
pub(crate) struct Info {
    /// The type of output to return
    #[clap(long, short, default_value = "print")]
    pub format: DisplayType,
}

#[derive(clap::ValueEnum, Clone, Debug, Deserialize, Serialize)]
pub(crate) enum DisplayType {
    Print,
    Json,
    Ron,
}

#[derive(clap::Args, Debug)]
pub(crate) struct Env {
    #[clap(subcommand)]
    pub env_type: EnvType,
}

#[derive(clap::Args, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct DotEnv {
    /// Whether or not to prepend `export` to each line
    #[clap(long, short)]
    pub export: bool,
}

#[derive(clap::Args, Clone, Debug, Deserialize, Serialize)]
pub(crate) struct DotEnvDirectory {
    /// The directory to write the environment variables to
    #[clap(long, short, default_value = ".direnv/env")]
    pub directory: PathBuf,
}

#[derive(clap::Subcommand, Clone, Debug, Deserialize, Serialize)]
pub(crate) enum EnvType {
    /// A .env file
    DotEnv(DotEnv),
    /// A directory with files for each environment variable
    Directory(DotEnvDirectory),
}
