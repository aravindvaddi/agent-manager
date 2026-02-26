use anyhow::{Context, Result};
use genai::chat::{ChatMessage, ChatOptions, ChatRequest};
use genai::Client;

use crate::model::agent::AgentConfig;
use crate::store::skill_store;

/// Build the system prompt by combining the agent's prompt with all attached skill prompts.
fn compile_system_prompt(agent: &AgentConfig) -> Result<String> {
    let mut parts = Vec::new();

    // Agent's own prompt
    if let Some(prompt) = &agent.prompt {
        if let Some(content) = &prompt.content {
            parts.push(content.clone());
        } else if let Some(file) = &prompt.file {
            let agents_dir = crate::store::paths::agents_dir()?;
            let path = agents_dir.join(file);
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                parts.push(content);
            }
        }
    }

    // Attached skill prompts
    if let Some(skills) = &agent.skills {
        if !skills.attached.is_empty() {
            parts.push("\n## Skills\n".to_string());
            for skill_name in &skills.attached {
                if let Ok(Some(prompt)) = skill_store::load_prompt(skill_name) {
                    parts.push(format!("### {skill_name}\n\n{prompt}\n"));
                }
            }
        }
    }

    if parts.is_empty() {
        Ok("You are a helpful assistant.".to_string())
    } else {
        Ok(parts.join("\n"))
    }
}

/// Build ChatOptions from agent config.
fn build_options(agent: &AgentConfig) -> Option<ChatOptions> {
    let opts = agent.options.as_ref()?;
    let mut chat_opts = ChatOptions::default();

    if let Some(temp) = opts.temperature {
        chat_opts = chat_opts.with_temperature(temp);
    }

    Some(chat_opts)
}

/// Execute an agent with a user prompt, returning the complete response.
pub async fn run(agent: &AgentConfig, user_prompt: &str) -> Result<String> {
    let system_prompt = compile_system_prompt(agent)?;
    let model = agent
        .model
        .as_deref()
        .unwrap_or("anthropic/claude-sonnet-4");

    let client = Client::default();

    let request = ChatRequest::default()
        .with_system(&system_prompt)
        .append_message(ChatMessage::user(user_prompt));

    let options = build_options(agent);
    let response = client
        .exec_chat(model, request, options.as_ref())
        .await
        .context("failed to execute chat request")?;

    let content = response
        .into_first_text()
        .unwrap_or_default();

    Ok(content)
}

/// Execute an agent with streaming output.
pub async fn run_stream(agent: &AgentConfig, user_prompt: &str) -> Result<String> {
    let system_prompt = compile_system_prompt(agent)?;
    let model = agent
        .model
        .as_deref()
        .unwrap_or("anthropic/claude-sonnet-4");

    let client = Client::default();

    let request = ChatRequest::default()
        .with_system(&system_prompt)
        .append_message(ChatMessage::user(user_prompt));

    let options = build_options(agent);

    // For now, streaming falls back to complete mode.
    // True streaming with exec_chat_stream can be added when needed.
    let response = client
        .exec_chat(model, request, options.as_ref())
        .await
        .context("failed to execute chat request")?;

    let content = response
        .into_first_text()
        .unwrap_or_default();

    Ok(content)
}
