use anyhow::{Context, Result};
use serde_json::json;

use crate::model::agent::AgentConfig;
use crate::store::skill_store;

const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
pub const DEFAULT_MODEL: &str = "anthropic/claude-sonnet-4.6";

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

/// Execute an agent with a user prompt, returning the complete response.
pub async fn run(agent: &AgentConfig, user_prompt: &str) -> Result<String> {
    let system_prompt = compile_system_prompt(agent)?;
    let model = agent.model.as_deref().unwrap_or(DEFAULT_MODEL);

    let api_key = std::env::var("OPENROUTER_API_KEY")
        .context("OPENROUTER_API_KEY must be set")?;

    let messages = vec![
        json!({"role": "system", "content": system_prompt}),
        json!({"role": "user", "content": user_prompt}),
    ];

    let mut body = json!({
        "model": model,
        "messages": messages,
    });

    if let Some(opts) = &agent.options {
        if let Some(temp) = opts.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = opts.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
    }

    let client = reqwest::Client::new();
    let response = client
        .post(OPENROUTER_URL)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .context("failed to send request to OpenRouter")?;

    let status = response.status();
    let response_body: serde_json::Value = response
        .json()
        .await
        .context("failed to parse OpenRouter response")?;

    if !status.is_success() {
        let error_msg = response_body["error"]["message"]
            .as_str()
            .unwrap_or("unknown error");
        anyhow::bail!("OpenRouter API error ({}): {}", status, error_msg);
    }

    let content = response_body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(content)
}

/// Execute an agent with streaming output.
/// For now, falls back to complete mode.
pub async fn run_stream(agent: &AgentConfig, user_prompt: &str) -> Result<String> {
    // TODO: implement true streaming with SSE
    run(agent, user_prompt).await
}
