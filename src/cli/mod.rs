pub mod agent;
pub mod run;
pub mod skill;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "am", about = "Agent Manager - manage and run LLM agents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage agents
    Agent {
        #[command(subcommand)]
        command: agent::AgentCommand,
    },
    /// Manage skills
    Skill {
        #[command(subcommand)]
        command: skill::SkillCommand,
    },
    /// Run an agent with a prompt
    Run(run::RunArgs),
}
