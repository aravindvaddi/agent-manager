use serde::{Deserialize, Serialize};

use super::skill::PromptSource;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentFile {
    pub agent: AgentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub description: Option<String>,
    pub model: Option<String>,
    pub options: Option<AgentOptions>,
    pub prompt: Option<PromptSource>,
    pub skills: Option<SkillsConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOptions {
    pub temperature: Option<f64>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsConfig {
    pub attached: Vec<String>,
}
