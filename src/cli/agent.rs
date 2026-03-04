use anyhow::Result;
use clap::Subcommand;

use crate::model::agent::{AgentConfig, AgentOptions};
use crate::model::skill::PromptSource;
use crate::output::{self, OutputMode};
use crate::store::agent_store;

#[derive(Subcommand)]
pub enum AgentCommand {
    /// Create a new agent
    Create {
        /// Agent name
        name: String,
        /// LLM model to use (e.g. "anthropic/claude-sonnet-4")
        #[arg(long)]
        model: Option<String>,
        /// Agent description
        #[arg(long)]
        description: Option<String>,
        /// System prompt (inline)
        #[arg(long)]
        prompt: Option<String>,
        /// Temperature (0.0-1.0)
        #[arg(long)]
        temperature: Option<f64>,
        /// Max tokens for response
        #[arg(long)]
        max_tokens: Option<u32>,
    },
    /// List all agents
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show agent details
    Show {
        /// Agent name
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete an agent
    Delete {
        /// Agent name
        name: String,
    },
    /// Update an existing agent's configuration
    Update {
        /// Agent name
        name: String,
        /// LLM model to use (e.g. "anthropic/claude-sonnet-4")
        #[arg(long)]
        model: Option<String>,
        /// Agent description
        #[arg(long)]
        description: Option<String>,
        /// System prompt (inline)
        #[arg(long)]
        prompt: Option<String>,
        /// Temperature (0.0-1.0)
        #[arg(long)]
        temperature: Option<f64>,
        /// Max tokens for response
        #[arg(long)]
        max_tokens: Option<u32>,
    },
}

pub fn handle(cmd: AgentCommand) -> Result<()> {
    match cmd {
        AgentCommand::Create {
            name,
            model,
            description,
            prompt,
            temperature,
            max_tokens,
        } => {
            let options = if temperature.is_some() || max_tokens.is_some() {
                Some(AgentOptions {
                    temperature,
                    max_tokens,
                })
            } else {
                None
            };

            let prompt_source = prompt.map(PromptSource::inline);

            let config = AgentConfig {
                name: name.clone(),
                description,
                model,
                options,
                prompt: prompt_source,
                skills: None,
            };

            agent_store::create(&config)?;
            println!("Created agent '{name}'");
            Ok(())
        }
        AgentCommand::List { json } => {
            let mode = if json {
                OutputMode::Json
            } else {
                OutputMode::Human
            };
            let agents = agent_store::list()?;
            output::output_agent_list(&agents, mode);
            Ok(())
        }
        AgentCommand::Show { name, json } => {
            let mode = if json {
                OutputMode::Json
            } else {
                OutputMode::Human
            };
            let agent = agent_store::load(&name)?;
            output::output_agent(&agent, mode);
            Ok(())
        }
        AgentCommand::Delete { name } => {
            agent_store::delete(&name)?;
            println!("Deleted agent '{name}'");
            Ok(())
        }
        AgentCommand::Update {
            name,
            model,
            description,
            prompt,
            temperature,
            max_tokens,
        } => {
            if model.is_none()
                && description.is_none()
                && prompt.is_none()
                && temperature.is_none()
                && max_tokens.is_none()
            {
                anyhow::bail!("provide at least one field to update");
            }

            let mut agent = agent_store::load(&name)?;

            if let Some(m) = model {
                agent.model = Some(m);
            }
            if let Some(d) = description {
                agent.description = Some(d);
            }
            if let Some(p) = prompt {
                agent.prompt = Some(PromptSource::inline(p));
            }
            if temperature.is_some() || max_tokens.is_some() {
                let opts = agent.options.get_or_insert(AgentOptions {
                    temperature: None,
                    max_tokens: None,
                });
                if let Some(t) = temperature {
                    opts.temperature = Some(t);
                }
                if let Some(mt) = max_tokens {
                    opts.max_tokens = Some(mt);
                }
            }

            agent_store::save(&agent)?;
            println!("Updated agent '{name}'");
            Ok(())
        }
    }
}
