use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use std::io::{self, Stdout};

use crate::chat::context::ChatContext;
use crate::llm::{create_prompt, SonnetClient};

/// Simple Terminal UI for the chat
pub struct ChatUi {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    input_buffer: String,
    messages: Vec<String>,
    scroll_offset: u16,
}

impl ChatUi {
    /// Create a new ChatUi
    pub fn new() -> Result<Self> {
        // Set up terminal
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            terminal,
            input_buffer: String::new(),
            messages: Vec::new(),
            scroll_offset: 0,
        })
    }
    
    /// Run the chat UI
    pub async fn run(&mut self, mut context: ChatContext) -> Result<()> {
        // Initialize LLM client
        let llm_client = SonnetClient::new()?;
        
        // Display welcome message
        self.messages.push("Welcome to miden-code! Type your question and press Enter to send.".to_string());
        self.messages.push("Press Ctrl+C to exit.".to_string());
        
        loop {
            // Draw the UI
            self.draw()?;
            
            // Handle input
            if let Event::Key(key) = event::read()? {
                match key {
                    // Exit on Ctrl+C
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => break,
                    
                    // Submit message on Enter
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        if !self.input_buffer.trim().is_empty() {
                            let user_message = self.input_buffer.clone();
                            self.messages.push(format!("You: {}", user_message));
                            self.input_buffer.clear();
                            
                            // Add message to context
                            context.add_user_message(user_message);
                            
                            // Display "thinking" message
                            self.messages.push("Assistant: Thinking...".to_string());
                            self.draw()?;
                            
                            // Create prompt and get response from LLM
                            let prompt = create_prompt(context.get_messages_clone());
                            match llm_client.send_message(prompt).await {
                                Ok(response) => {
                                    // Remove "thinking" message
                                    self.messages.pop();
                                    
                                    // Add response to messages and context
                                    self.messages.push(format!("Assistant: {}", response));
                                    context.add_assistant_message(response);
                                }
                                Err(e) => {
                                    // Remove "thinking" message
                                    self.messages.pop();
                                    
                                    // Display error message
                                    self.messages.push(format!("Error: {}", e));
                                }
                            }
                        }
                    }
                    
                    // Handle backspace
                    KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        self.input_buffer.pop();
                    }
                    
                    // Handle regular character input
                    KeyEvent {
                        code: KeyCode::Char(c),
                        ..
                    } => {
                        self.input_buffer.push(c);
                    }
                    
                    // Scroll up
                    KeyEvent {
                        code: KeyCode::Up,
                        ..
                    } => {
                        if self.scroll_offset > 0 {
                            self.scroll_offset -= 1;
                        }
                    }
                    
                    // Scroll down
                    KeyEvent {
                        code: KeyCode::Down,
                        ..
                    } => {
                        self.scroll_offset += 1;
                    }
                    
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Draw the UI
    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            let size = f.size();
            
            // Calculate layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);
            
            // Draw messages area
            let messages_text = self.messages.join("\n");
            let messages_paragraph = Paragraph::new(messages_text)
                .block(Block::default().borders(Borders::ALL).title("Chat"))
                .wrap(Wrap { trim: true })
                .scroll((self.scroll_offset, 0));
            
            f.render_widget(messages_paragraph, chunks[0]);
            
            // Draw input area
            let input_paragraph = Paragraph::new(self.input_buffer.as_str())
                .block(Block::default().borders(Borders::ALL).title("Input"));
            
            f.render_widget(input_paragraph, chunks[1]);
            
            // Position cursor in input field
            f.set_cursor(
                chunks[1].x + self.input_buffer.len() as u16 + 1,
                chunks[1].y + 1,
            );
        })?;
        
        Ok(())
    }
}

impl Drop for ChatUi {
    fn drop(&mut self) {
        // Restore terminal
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            cursor::Show
        );
    }
}