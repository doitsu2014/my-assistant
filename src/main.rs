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
    
    // Run bot and perform sync operations
    if let Err(e) = bot.run().await {
        eprintln!("Error running bot: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}
