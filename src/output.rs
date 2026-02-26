use crate::model::agent::AgentConfig;
use crate::model::skill::SkillConfig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Human,
    Json,
}

pub fn output_agent(agent: &AgentConfig, mode: OutputMode) {
    match mode {
        OutputMode::Human => {
            println!("Agent: {}", agent.name);
            if let Some(desc) = &agent.description {
                println!("  Description: {desc}");
            }
            if let Some(model) = &agent.model {
                println!("  Model: {model}");
            }
            if let Some(opts) = &agent.options {
                if let Some(temp) = opts.temperature {
                    println!("  Temperature: {temp}");
                }
                if let Some(max) = opts.max_tokens {
                    println!("  Max tokens: {max}");
                }
            }
            if let Some(skills) = &agent.skills {
                if !skills.attached.is_empty() {
                    println!("  Skills: {}", skills.attached.join(", "));
                }
            }
        }
        OutputMode::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(agent).expect("failed to serialize agent")
            );
        }
    }
}

pub fn output_agent_list(agents: &[AgentConfig], mode: OutputMode) {
    match mode {
        OutputMode::Human => {
            if agents.is_empty() {
                println!("No agents configured.");
                return;
            }
            for agent in agents {
                let desc = agent
                    .description
                    .as_deref()
                    .unwrap_or("(no description)");
                let model = agent.model.as_deref().unwrap_or("(default)");
                println!("  {} - {} [{}]", agent.name, desc, model);
            }
        }
        OutputMode::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(agents).expect("failed to serialize agents")
            );
        }
    }
}

pub fn output_skill(skill: &SkillConfig, mode: OutputMode) {
    match mode {
        OutputMode::Human => {
            println!("Skill: {}", skill.name);
            if let Some(desc) = &skill.description {
                println!("  Description: {desc}");
            }
            if let Some(version) = &skill.version {
                println!("  Version: {version}");
            }
            if let Some(tags) = &skill.tags {
                if !tags.is_empty() {
                    println!("  Tags: {}", tags.join(", "));
                }
            }
        }
        OutputMode::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(skill).expect("failed to serialize skill")
            );
        }
    }
}

pub fn output_skill_list(skills: &[SkillConfig], mode: OutputMode) {
    match mode {
        OutputMode::Human => {
            if skills.is_empty() {
                println!("No skills configured.");
                return;
            }
            for skill in skills {
                let desc = skill
                    .description
                    .as_deref()
                    .unwrap_or("(no description)");
                println!("  {} - {}", skill.name, desc);
            }
        }
        OutputMode::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(skills).expect("failed to serialize skills")
            );
        }
    }
}
