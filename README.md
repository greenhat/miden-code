# miden-code

An AI Coding Assistant CLI using Sonnet 3.7 with file system and shell access capabilities.

## Features

- Terminal-based chat UI for natural language interaction with the AI
- Support for multi-turn conversations with context retention
- Integration with Sonnet 3.7 LLM for advanced reasoning capabilities
- File system access for reading code and understanding context
- Shell command execution (with explicit permission)

## Installation

### Prerequisites

- Rust and Cargo (1.75 or later)
- OpenRouter API key (get one from https://openrouter.ai/keys)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/miden-code.git
   cd miden-code
   ```

2. Create a `.env` file with your OpenRouter API key:
   ```bash
   cp .env.example .env
   # Edit .env and add your API key
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Start a new chat session:

```bash
./target/release/miden-code
```

Start with an initial prompt:

```bash
./target/release/miden-code chat -p "How do I use async/await in Rust?"
```

In the chat interface:
- Type your question and press Enter to send
- Press Ctrl+C to exit
- Use Up/Down arrows to scroll through the chat history