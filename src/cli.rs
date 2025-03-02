use anyhow::Result;
use clap::{Parser, Subcommand};

/// AI Coding Assistant CLI using Sonnet 3.7
#[derive(Debug, Parser)]
#[command(name = "miden-code")]
#[command(about = "AI Coding Assistant CLI using Sonnet 3.7 with file system and shell access capabilities")]
#[command(version)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

/// Supported commands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start a new chat session
    Chat {
        /// Initial prompt to start the conversation
        #[arg(short, long)]
        prompt: Option<String>,
    },
}

/// Parse command line arguments
pub fn parse_args() -> Cli {
    Cli::parse()
}

/// Run the CLI application
pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Chat { prompt }) => {
            // Initialize the chat session
            let initial_prompt = prompt.unwrap_or_else(|| String::new());
            crate::chat::start_session(initial_prompt).await?;
        }
        None => {
            // Default to chat mode without an initial prompt
            crate::chat::start_session(String::new()).await?;
        }
    }

    Ok(())
}