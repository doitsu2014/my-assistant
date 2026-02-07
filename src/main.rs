mod bot;
mod config;
mod github;
mod notion;

use anyhow::Result;
use bot::OpenClawBot;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            eprintln!("Please create a .env file based on .env.example and fill in the required values.");
            std::process::exit(1);
        }
    };
    
    // Initialize bot
    let bot = match OpenClawBot::new(config) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error initializing bot: {}", e);
            std::process::exit(1);
        }
    };
    
    // Run bot
    bot.run().await?;
    
    // Example: Show current tasks and issues
    if let Err(e) = bot.sync_notion_to_github() {
        eprintln!("Error syncing Notion to GitHub: {}", e);
    }
    
    if let Err(e) = bot.sync_github_to_notion().await {
        eprintln!("Error syncing GitHub to Notion: {}", e);
    }
    
    Ok(())
}
