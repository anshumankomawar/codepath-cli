mod git;
mod projects;
mod setup;

use clap::{Parser, Subcommand};
use keyring::Entry;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, PartialEq)]
enum Commands {
    Auth,
    Setup,
    Init { project: String },
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let entry = Entry::new("codepath", "auth").unwrap();
    if entry.get_password().is_err() && cli.command != Commands::Auth {
        git::authenticate_user().await;
    }

    match &cli.command {
        Commands::Auth => {
            git::authenticate_user().await;
        }
        Commands::Setup => {
            setup::setup();
        }
        Commands::Init { project } => {
            projects::init(project);
        }
        Commands::List => projects::list(),
    }
}
