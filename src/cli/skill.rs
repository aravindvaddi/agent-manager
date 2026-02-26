use anyhow::Result;
use clap::Subcommand;

use crate::model::skill::SkillConfig;
use crate::output::{self, OutputMode};
use crate::store::skill_store;

#[derive(Subcommand)]
pub enum SkillCommand {
    /// Create a new skill
    Create {
        /// Skill name
        name: String,
        /// Skill description
        #[arg(long)]
        description: Option<String>,
        /// Skill version
        #[arg(long)]
        version: Option<String>,
        /// Tags (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// List all skills
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show skill details
    Show {
        /// Skill name
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a skill
    Delete {
        /// Skill name
        name: String,
    },
    /// Attach a skill to an agent
    Attach {
        /// Skill name
        skill: String,
        /// Agent to attach to
        #[arg(long = "to")]
        agent: String,
    },
    /// Detach a skill from an agent
    Detach {
        /// Skill name
        skill: String,
        /// Agent to detach from
        #[arg(long = "from")]
        agent: String,
    },
}

pub fn handle(cmd: SkillCommand) -> Result<()> {
    match cmd {
        SkillCommand::Create {
            name,
            description,
            version,
            tags,
        } => {
            let config = SkillConfig {
                name: name.clone(),
                description,
                version: version.or(Some("0.1.0".to_string())),
                tags,
                prompt: None,
            };

            skill_store::create(&config)?;
            println!("Created skill '{name}'");
            Ok(())
        }
        SkillCommand::List { json } => {
            let mode = if json {
                OutputMode::Json
            } else {
                OutputMode::Human
            };
            let skills = skill_store::list()?;
            output::output_skill_list(&skills, mode);
            Ok(())
        }
        SkillCommand::Show { name, json } => {
            let mode = if json {
                OutputMode::Json
            } else {
                OutputMode::Human
            };
            let skill = skill_store::load(&name)?;
            output::output_skill(&skill, mode);
            Ok(())
        }
        SkillCommand::Delete { name } => {
            skill_store::delete(&name)?;
            println!("Deleted skill '{name}'");
            Ok(())
        }
        SkillCommand::Attach { skill, agent } => {
            skill_store::attach(&skill, &agent)?;
            println!("Attached skill '{skill}' to agent '{agent}'");
            Ok(())
        }
        SkillCommand::Detach { skill, agent } => {
            skill_store::detach(&skill, &agent)?;
            println!("Detached skill '{skill}' from agent '{agent}'");
            Ok(())
        }
    }
}
