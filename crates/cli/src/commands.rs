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
    Info,
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
pub(crate) struct Info {}
