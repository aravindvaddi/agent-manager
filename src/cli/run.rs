use anyhow::{Context, Result};
use clap::Args;

use crate::engine::runtime::{self, DEFAULT_MODEL};
use crate::store::agent_store;

#[derive(Args)]
pub struct RunArgs {
    /// Agent name
    pub agent: String,

    /// Prompt text (provide either this or --file)
    pub prompt: Option<String>,

    /// Read prompt from a file
    #[arg(long)]
    pub file: Option<String>,

    /// Stream output in real-time
    #[arg(long)]
    pub stream: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

pub async fn handle(args: RunArgs) -> Result<()> {
    let agent = agent_store::load(&args.agent)?;

    // Resolve prompt from args or file
    let prompt = if let Some(text) = &args.prompt {
        text.clone()
    } else if let Some(path) = &args.file {
        std::fs::read_to_string(path)
            .with_context(|| format!("failed to read prompt file: {path}"))?
    } else {
        anyhow::bail!("provide a prompt or use --file <path>");
    };

    let response = if args.stream {
        runtime::run_stream(&agent, &prompt).await?
    } else {
        runtime::run(&agent, &prompt).await?
    };

    if args.json {
        let output = serde_json::json!({
            "agent": args.agent,
            "model": agent.model.as_deref().unwrap_or(DEFAULT_MODEL),
            "prompt": prompt,
            "response": response,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        print!("{response}");
    }

    Ok(())
}
