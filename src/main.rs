mod projects;
mod setup;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Init { project: projects::Projects },
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Setup => {
            setup::setup();
        }
        Commands::Init { project } => projects::init(project),
        Commands::List => projects::list(),
    }
}
