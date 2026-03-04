use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillFile {
    pub skill: SkillConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub tags: Option<Vec<String>>,
    pub prompt: Option<PromptSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSource {
    pub file: Option<String>,
    pub content: Option<String>,
}

impl PromptSource {
    pub fn inline(content: String) -> Self {
        Self {
            file: None,
            content: Some(content),
        }
    }
}
