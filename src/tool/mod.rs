pub mod builtin;

use async_trait::async_trait;

#[async_trait]
pub trait ToolProvider: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value) -> anyhow::Result<String>;
}
