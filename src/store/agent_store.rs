use anyhow::{bail, Context, Result};

use crate::model::agent::{AgentConfig, AgentFile};
use crate::store::paths;

/// Create a new agent config file.
pub fn create(config: &AgentConfig) -> Result<()> {
    let path = paths::agent_path(&config.name)?;
    if path.exists() {
        bail!("agent '{}' already exists", config.name);
    }
    save(config)
}

/// Load an agent config by name.
pub fn load(name: &str) -> Result<AgentConfig> {
    let path = paths::agent_path(name)?;
    if !path.exists() {
        bail!("agent '{name}' not found");
    }

    let content =
        std::fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let file: AgentFile =
        toml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))?;

    Ok(file.agent)
}

/// Save an agent config (overwriting existing).
pub fn save(config: &AgentConfig) -> Result<()> {
    let dir = paths::agents_dir()?;
    paths::ensure_dir(&dir)?;

    let path = paths::agent_path(&config.name)?;
    let file = AgentFile {
        agent: config.clone(),
    };
    let content = toml::to_string_pretty(&file).context("failed to serialize agent config")?;
    std::fs::write(&path, content)
        .with_context(|| format!("failed to write {}", path.display()))?;

    Ok(())
}

/// List all agent configs.
pub fn list() -> Result<Vec<AgentConfig>> {
    let dir = paths::agents_dir()?;
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut agents = Vec::new();
    for entry in std::fs::read_dir(&dir).context("failed to read agents directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            match toml::from_str::<AgentFile>(&content) {
                Ok(file) => agents.push(file.agent),
                Err(e) => {
                    tracing::warn!("skipping {}: {e}", path.display());
                }
            }
        }
    }

    agents.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(agents)
}

/// Delete an agent config by name.
pub fn delete(name: &str) -> Result<()> {
    let path = paths::agent_path(name)?;
    if !path.exists() {
        bail!("agent '{name}' not found");
    }

    std::fs::remove_file(&path).with_context(|| format!("failed to delete {}", path.display()))?;
    Ok(())
}
