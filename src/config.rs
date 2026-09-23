use anyhow::Ok;
use clap::Subcommand;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{Cli, auth::{dropbox_auth, gdrive_auth}, providers};

#[derive(Subcommand)]
pub enum Commands {
    Config,
    Upload { path: String },
}

enum ConfigActions {
    SetDropBox,
    SetGDrive,
    Exit,
}

pub async fn config(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Config => {
            let config_selector = vec!["Connect GDrive", "Connect DropBox", "Exit"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What action you want to perform ?")
                .default(0)
                .items(&config_selector)
                .interact()
                .expect("Failed to get selection");

            let action = match selection {
                0 => ConfigActions::SetGDrive,
                1 => ConfigActions::SetDropBox,
                _ => ConfigActions::Exit,
            };

            match action {
                ConfigActions::SetGDrive => {
                    gdrive_auth().await.expect("failed to auth");
                }
                ConfigActions::SetDropBox => {
                    dropbox_auth();
                }
                ConfigActions::Exit => {
                    println!("NeverMind!")
                }
            }
        }

        Commands::Upload { path } => {
            println!("Uploading: {path}");
        
            let file = providers::upload_file(
                std::path::Path::new(&path)
            )
            .await?;
        
            println!("Uploaded successfully!");
            println!("Name: {}", file.name);
            println!("ID: {}", file.id);
        }
    }

    Ok(())
}
