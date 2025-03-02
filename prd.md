# Product Requirements Document: `miden-code` AI Coding Assistant CLI

## Overview
The AI Coding Assistant CLI is a command-line tool written in Rust that provides developers with an AI-powered coding companion. It leverages the Sonnet 3.7 thinking mode to deliver intelligent assistance directly in the terminal, enabling users to interact with the AI through a chat interface while granting it limited filesystem and shell access capabilities.

## Key Features

### 1. Chat Interface
- Terminal-based chat UI for natural language interaction with the AI
- Support for multi-turn conversations with context retention

### 2. Sonnet 3.7 Thinking Mode
- Integration with Sonnet 3.7 LLM for advanced reasoning capabilities
- Explicit step-by-step reasoning for complex problems
- Chain-of-thought processing for debugging and problem-solving
- Self-correction mechanisms when solutions have errors
- Citation of sources when providing reference information

### 3. LLM File System Commands
The following commands will be exposed to the LLM as tools that the LLM will use as function calls. These function calls will allow the LLM to interact with the file system and execute commands as needed during the conversation.

#### 3.1 List Directory Command
- Allow the LLM to request directory listings (run shell `ls` command)

#### 3.2 Read File Command
- Enable the LLM to read file contents for analysis

#### 3.3 Execute Shell Command
- Allow the LLM to suggest and run shell commands
- Preview of command before execution (with user confirmation)
- Capture and display command output for analysis

### Security
- All LLM operations must be authorized by the user
- No automatic execution of suggested commands

