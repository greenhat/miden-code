use crate::llm::ChatMessage;

/// Maximum number of messages to keep in context
const MAX_CONTEXT_MESSAGES: usize = 20;

/// Manages the chat conversation context
pub struct ChatContext {
    messages: Vec<ChatMessage>,
}

impl ChatContext {
    /// Create a new chat context
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Add a user message to the context
    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(ChatMessage::user(content));
        self.prune_if_needed();
    }

    /// Add an assistant message to the context
    pub fn add_assistant_message(&mut self, content: String) {
        self.messages.push(ChatMessage::assistant(content));
        self.prune_if_needed();
    }

    /// Get all messages in the context
    pub fn get_messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    /// Get a clone of all messages in the context
    pub fn get_messages_clone(&self) -> Vec<ChatMessage> {
        self.messages.clone()
    }

    /// Prune the context if it exceeds the maximum size
    fn prune_if_needed(&mut self) {
        if self.messages.len() > MAX_CONTEXT_MESSAGES {
            // Remove oldest messages, but keep system messages
            let to_remove = self.messages.len() - MAX_CONTEXT_MESSAGES;
            
            // Skip system messages (role == "system") when pruning
            let mut removed = 0;
            let mut i = 0;
            
            while removed < to_remove && i < self.messages.len() {
                if self.messages[i].role != "system" {
                    self.messages.remove(i);
                    removed += 1;
                } else {
                    i += 1;
                }
            }
        }
    }
}