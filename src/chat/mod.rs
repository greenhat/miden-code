pub mod context;
pub mod ui;

use anyhow::Result;
use context::ChatContext;
use ui::ChatUi;

/// Start a new chat session
pub async fn start_session(initial_prompt: String) -> Result<()> {
    let mut chat_context = ChatContext::new();
    let mut chat_ui = ChatUi::new()?;
    
    // Add the initial prompt if provided
    if !initial_prompt.is_empty() {
        chat_context.add_user_message(initial_prompt);
    }
    
    // Start the chat UI
    chat_ui.run(chat_context).await?;
    
    Ok(())
}