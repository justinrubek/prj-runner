use crate::commands::Commands;
use clap::Parser;
use std::error::Error;
use tracing::{debug, info};

mod commands;
mod error;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Project(project) => {
            let project_root = prj_base_directory::get_project_root().await?;

            match project.command {
                commands::ProjectCommands::Exec(exec) => {
                    debug!(?project_root, ?exec, "Running command in project root");

                    let mut process = tokio::process::Command::new(exec.command)
                        .current_dir(project_root.unwrap())
                        .args(exec.args)
                        .spawn()?;

                    let status = process.wait().await?;
                    if !status.success() {
                        let code = status.code().expect("no exit code");
                        return Err(error::Error::ExecError(code).into());
                    }
                }
                commands::ProjectCommands::Info => {
                    info!(?project_root);
                }
            }
        }
    }

    Ok(())
}
