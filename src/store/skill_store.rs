use anyhow::{bail, Context, Result};

use crate::model::skill::{SkillConfig, SkillFile};
use crate::store::paths;

/// Create a new skill with its directory and config.
pub fn create(config: &SkillConfig) -> Result<()> {
    let dir = paths::skill_dir(&config.name)?;
    if dir.exists() {
        bail!("skill '{}' already exists", config.name);
    }
    paths::ensure_dir(&dir)?;

    let file = SkillFile {
        skill: config.clone(),
    };
    let content = toml::to_string_pretty(&file).context("failed to serialize skill config")?;
    let path = paths::skill_path(&config.name)?;
    std::fs::write(&path, content)
        .with_context(|| format!("failed to write {}", path.display()))?;

    // Create default prompt.md
    let prompt_path = paths::skill_prompt_path(&config.name)?;
    let desc = config.description.as_deref().unwrap_or(&config.name);
    std::fs::write(
        &prompt_path,
        format!("# {}\n\n{desc}\n", config.name),
    )
    .with_context(|| format!("failed to write {}", prompt_path.display()))?;

    Ok(())
}

/// Load a skill config by name.
pub fn load(name: &str) -> Result<SkillConfig> {
    let path = paths::skill_path(name)?;
    if !path.exists() {
        bail!("skill '{name}' not found");
    }

    let content =
        std::fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let file: SkillFile =
        toml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))?;

    Ok(file.skill)
}

/// List all skill configs.
pub fn list() -> Result<Vec<SkillConfig>> {
    let dir = paths::skills_dir()?;
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut skills = Vec::new();
    for entry in std::fs::read_dir(&dir).context("failed to read skills directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let skill_toml = path.join("skill.toml");
            if skill_toml.exists() {
                let content = std::fs::read_to_string(&skill_toml)
                    .with_context(|| format!("failed to read {}", skill_toml.display()))?;
                match toml::from_str::<SkillFile>(&content) {
                    Ok(file) => skills.push(file.skill),
                    Err(e) => {
                        tracing::warn!("skipping {}: {e}", skill_toml.display());
                    }
                }
            }
        }
    }

    skills.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(skills)
}

/// Delete a skill by name (removes entire directory).
pub fn delete(name: &str) -> Result<()> {
    let dir = paths::skill_dir(name)?;
    if !dir.exists() {
        bail!("skill '{name}' not found");
    }

    std::fs::remove_dir_all(&dir)
        .with_context(|| format!("failed to delete {}", dir.display()))?;
    Ok(())
}

/// Load the prompt content for a skill, resolving file references.
pub fn load_prompt(name: &str) -> Result<Option<String>> {
    let config = load(name)?;

    if let Some(prompt) = &config.prompt {
        // Inline content takes priority
        if let Some(content) = &prompt.content {
            return Ok(Some(content.clone()));
        }
        // File reference
        if let Some(file) = &prompt.file {
            let skill_dir = paths::skill_dir(name)?;
            let prompt_path = skill_dir.join(file);
            if prompt_path.exists() {
                let content = std::fs::read_to_string(&prompt_path)
                    .with_context(|| format!("failed to read {}", prompt_path.display()))?;
                return Ok(Some(content));
            }
        }
    }

    // Fallback: try default prompt.md
    let prompt_path = paths::skill_prompt_path(name)?;
    if prompt_path.exists() {
        let content = std::fs::read_to_string(&prompt_path)
            .with_context(|| format!("failed to read {}", prompt_path.display()))?;
        return Ok(Some(content));
    }

    Ok(None)
}

/// Attach a skill to an agent by updating the agent's config.
pub fn attach(skill_name: &str, agent_name: &str) -> Result<()> {
    // Verify skill exists
    let _skill = load(skill_name)?;

    // Load and update agent
    let mut agent = crate::store::agent_store::load(agent_name)?;
    let skills = agent.skills.get_or_insert_with(|| crate::model::agent::SkillsConfig {
        attached: Vec::new(),
    });

    if skills.attached.contains(&skill_name.to_string()) {
        bail!("skill '{skill_name}' is already attached to agent '{agent_name}'");
    }

    skills.attached.push(skill_name.to_string());
    crate::store::agent_store::save(&agent)?;

    Ok(())
}

/// Detach a skill from an agent.
pub fn detach(skill_name: &str, agent_name: &str) -> Result<()> {
    let mut agent = crate::store::agent_store::load(agent_name)?;
    let skills = agent.skills.as_mut();

    match skills {
        Some(skills) => {
            let pos = skills
                .attached
                .iter()
                .position(|s| s == skill_name);
            match pos {
                Some(idx) => {
                    skills.attached.remove(idx);
                    crate::store::agent_store::save(&agent)?;
                    Ok(())
                }
                None => bail!("skill '{skill_name}' is not attached to agent '{agent_name}'"),
            }
        }
        None => bail!("agent '{agent_name}' has no skills attached"),
    }
}
