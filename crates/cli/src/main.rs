use crate::commands::{Commands, DotEnv, DotEnvDirectory, EnvType};
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
                        .current_dir(project.root_directory.unwrap())
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
                        println!("{}", serde_json::to_string_pretty(&project)?);
                    }
                    DisplayType::Ron => {
                        let value = ron::ser::to_string_pretty(&project, Default::default())?;
                        println!("{}", value);
                    }
                },
                commands::ProjectCommands::GenerateEnv(env) => {
                    let env_vars = project.project_hashmap();

                    match env.env_type {
                        EnvType::DotEnv(DotEnv { export }) => {
                            for (key, value) in env_vars {
                                if let Some(value) = value {
                                    if export {
                                        println!("export {}={}", key, value);
                                    } else {
                                        println!("{}={}", key, value);
                                    }
                                }
                            }
                        }
                        EnvType::Directory(DotEnvDirectory { directory }) => {
                            // ensure the directory exists
                            tokio::fs::create_dir_all(&directory).await?;

                            for (key, value) in env_vars {
                                if let Some(value) = value {
                                    let file_path = directory.join(key);
                                    tokio::fs::write(file_path, value).await?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
