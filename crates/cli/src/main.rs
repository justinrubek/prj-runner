use crate::commands::Commands;
use clap::Parser;
use commands::DisplayType;
use project_base_directory::Project;
use std::error::Error;
use tracing::{debug, info};

mod commands;
mod error;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Project(project_cmd) => {
            let project = Project::discover_and_assume().await?;

            match project_cmd.command {
                commands::ProjectCommands::Exec(exec) => {
                    debug!(?project, ?exec, "Running command in project");

                    let mut process = tokio::process::Command::new(exec.command)
                        .current_dir(&project.root_directory.unwrap())
                        .args(exec.args)
                        .spawn()?;

                    let status = process.wait().await?;
                    if !status.success() {
                        let code = status.code().expect("no exit code");
                        return Err(error::Error::ExecError(code).into());
                    }
                }
                commands::ProjectCommands::Info(info) => match info.format {
                    DisplayType::Print => {
                        info!(?project);
                    }
                    DisplayType::Json => {
                        let value = serde_json::json!(project);
                        println!("{}", serde_json::to_string_pretty(&value)?);
                    }
                    DisplayType::Ron => {
                        let value = ron::ser::to_string_pretty(&project, Default::default())?;
                        println!("{}", value);
                    }
                },
            }
        }
    }

    Ok(())
}
