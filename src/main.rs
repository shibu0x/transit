use clap::Parser;

mod config;
mod dropbox;
mod gdrive;

#[derive(Parser)]
#[command(name = "forgeq")]
#[command(about = "A cli tool to upload any data to any kind of storage from terminal")]
pub struct Cli {
    #[command(subcommand)]
    command: config::Commands
}

fn main() {
    let cli = Cli::parse();
    config::config(cli);
}
