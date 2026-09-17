use clap::Subcommand;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{Cli, dropbox::dropbox_auth, gdrive::gdrive_auth};

#[derive(Subcommand)]
pub enum Commands {
    Config,
}

enum ConfigActions {
    SetDropBox,
    SetGDrive,
    Exit,
}

pub fn config(cli: Cli) {
    match cli.command {
        Commands::Config => {
            let config_selector = vec!["Connect DropBox", "Connect GDrive", "Exit"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What action you want to perform ?")
                .default(0)
                .items(&config_selector)
                .interact()
                .expect("Failed to get selection");

            let action = match selection {
                0 => ConfigActions::SetDropBox,
                1 => ConfigActions::SetGDrive,
                _ => ConfigActions::Exit,
            };

            match action {
                ConfigActions::SetDropBox => {
                    dropbox_auth();
                }
                ConfigActions::SetGDrive => {
                    gdrive_auth();
                }
                ConfigActions::Exit => {
                    println!("NeverMind!")
                }
            }
        }
    }
}
