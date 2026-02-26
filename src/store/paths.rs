use std::path::PathBuf;

use anyhow::{Context, Result};

/// Base config directory: ~/.config/am/
pub fn config_dir() -> Result<PathBuf> {
    let base = dirs::config_dir().context("could not determine config directory")?;
    Ok(base.join("am"))
}

/// Directory for agent configs: ~/.config/am/agents/
pub fn agents_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("agents"))
}

/// Path to a specific agent config: ~/.config/am/agents/<name>.toml
pub fn agent_path(name: &str) -> Result<PathBuf> {
    Ok(agents_dir()?.join(format!("{name}.toml")))
}

/// Directory for skill configs: ~/.config/am/skills/
pub fn skills_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("skills"))
}

/// Directory for a specific skill: ~/.config/am/skills/<name>/
pub fn skill_dir(name: &str) -> Result<PathBuf> {
    Ok(skills_dir()?.join(name))
}

/// Path to a specific skill config: ~/.config/am/skills/<name>/skill.toml
pub fn skill_path(name: &str) -> Result<PathBuf> {
    Ok(skill_dir(name)?.join("skill.toml"))
}

/// Path to a skill's prompt file: ~/.config/am/skills/<name>/prompt.md
pub fn skill_prompt_path(name: &str) -> Result<PathBuf> {
    Ok(skill_dir(name)?.join("prompt.md"))
}

/// Global config path: ~/.config/am/config.toml
pub fn global_config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

/// Ensure a directory exists, creating it if necessary.
pub fn ensure_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .with_context(|| format!("failed to create directory: {}", path.display()))?;
    }
    Ok(())
}
