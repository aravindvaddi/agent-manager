# am - Agent Manager

A CLI tool for managing and running LLM agents. Each agent has a specialized set of skills and can be executed with prompts against any LLM provider.

## Features

- **Multi-provider LLM support** via [genai](https://github.com/jeremychone/rust-genai) (OpenAI, Anthropic, Google, Ollama, etc.)
- **Skills system** - modular prompt components that can be attached to any agent
- **TOML + Markdown config** - structured agent configs with Markdown prompt files
- **JSON output** - `--json` flag on all commands for scripting and piping
- **XDG-compliant** - configs stored in `~/.config/am/`

## Installation

Requires [Rust](https://rustup.rs/) (1.85+ for edition 2024).

```bash
# Clone and install
git clone https://github.com/YOUR_USERNAME/am.git
cd am
./install.sh

# Or manually
cargo install --path .
```

## Quick Start

```bash
# Set an API key for your provider
export ANTHROPIC_API_KEY="sk-..."

# Create an agent
am agent create researcher \
  --model "anthropic/claude-sonnet-4" \
  --description "Researches topics and synthesizes information"

# Create and attach a skill
am skill create summarize --description "Summarize information concisely"
am skill attach summarize --to researcher

# Run the agent
am run researcher "What are the key differences between TCP and UDP?"

# JSON output for scripting
am run researcher "Explain REST APIs" --json
```

## Usage

### Agents

Agents are LLM configurations with a model, system prompt, and attached skills.

```bash
am agent create <name> [--model <model>] [--description <desc>] [--prompt <text>]
am agent list [--json]
am agent show <name> [--json]
am agent delete <name>
```

### Skills

Skills are reusable prompt modules stored as Markdown files. Attach them to agents to compose behavior.

```bash
am skill create <name> [--description <desc>] [--tags <t1,t2>]
am skill list [--json]
am skill show <name> [--json]
am skill delete <name>
am skill attach <skill> --to <agent>
am skill detach <skill> --from <agent>
```

### Running Agents

```bash
am run <agent> "<prompt>"
am run <agent> --file <prompt-file>
am run <agent> "<prompt>" --stream
am run <agent> "<prompt>" --json
```

## Configuration

### Agent Config (`~/.config/am/agents/<name>.toml`)

```toml
[agent]
name = "researcher"
description = "Researches topics and synthesizes information"
model = "anthropic/claude-sonnet-4"

[agent.options]
temperature = 0.7
max_tokens = 4096

[agent.prompt]
content = "You are a research assistant..."
# Or reference a file:
# file = "researcher.md"

[agent.skills]
attached = ["summarize", "web-search"]
```

### Skill Config (`~/.config/am/skills/<name>/skill.toml`)

```toml
[skill]
name = "summarize"
description = "Summarize information concisely"
version = "0.1.0"
tags = ["writing", "research"]
```

The skill's prompt lives in `~/.config/am/skills/<name>/prompt.md` and is automatically appended to the agent's system prompt when attached.

## Supported Models

Any model supported by the [genai crate](https://github.com/jeremychone/rust-genai), including:

| Provider | Example Model | Env Variable |
|----------|--------------|--------------|
| Anthropic | `anthropic/claude-sonnet-4` | `ANTHROPIC_API_KEY` |
| OpenAI | `openai/gpt-4o` | `OPENAI_API_KEY` |
| Google | `google/gemini-2.0-flash` | `GEMINI_API_KEY` |
| Ollama | `ollama/llama3` | (local, no key needed) |

## License

MIT
