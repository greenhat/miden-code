mod cli;
mod chat;
mod llm;
mod tools;

use anyhow::Result;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Load .env file if present
    dotenv::dotenv().ok();
    
    info!("Starting miden-code...");
    
    // Parse command line arguments and run the application
    let cli = cli::parse_args();
    cli::run(cli).await?;
    
    Ok(())
}
