/// Configuration management for OpenClaw Bot
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // Notion Configuration
    pub notion_api_key: String,
    pub notion_database_id: String,
    
    // GitHub Configuration
    pub github_token: String,
    pub github_owner: String,
    pub github_repo: String,
    
    // Bot Configuration
    pub bot_name: String,
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();
        
        let config = Config {
            notion_api_key: env::var("NOTION_API_KEY")
                .map_err(|_| ConfigError::MissingVar("NOTION_API_KEY".to_string()))?,
            notion_database_id: env::var("NOTION_DATABASE_ID")
                .map_err(|_| ConfigError::MissingVar("NOTION_DATABASE_ID".to_string()))?,
            github_token: env::var("GITHUB_TOKEN")
                .map_err(|_| ConfigError::MissingVar("GITHUB_TOKEN".to_string()))?,
            github_owner: env::var("GITHUB_OWNER")
                .map_err(|_| ConfigError::MissingVar("GITHUB_OWNER".to_string()))?,
            github_repo: env::var("GITHUB_REPO")
                .map_err(|_| ConfigError::MissingVar("GITHUB_REPO".to_string()))?,
            bot_name: env::var("BOT_NAME").unwrap_or_else(|_| "OpenClaw Assistant".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()),
        };
        
        Ok(config)
    }
    
    /// Validate that all required configuration is present
    pub fn validate(&self) -> Vec<String> {
        let mut missing = Vec::new();
        
        if self.notion_api_key.is_empty() {
            missing.push("NOTION_API_KEY".to_string());
        }
        if self.notion_database_id.is_empty() {
            missing.push("NOTION_DATABASE_ID".to_string());
        }
        if self.github_token.is_empty() {
            missing.push("GITHUB_TOKEN".to_string());
        }
        if self.github_owner.is_empty() {
            missing.push("GITHUB_OWNER".to_string());
        }
        if self.github_repo.is_empty() {
            missing.push("GITHUB_REPO".to_string());
        }
        
        missing
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
}
