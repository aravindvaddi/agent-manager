mod cli;
mod engine;
mod model;
mod output;
mod store;
mod tool;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing (controlled via RUST_LOG env var)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Agent { command } => cli::agent::handle(command),
        Commands::Skill { command } => cli::skill::handle(command),
        Commands::Run(args) => cli::run::handle(args).await,
    }
}
