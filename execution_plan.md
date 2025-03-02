# miden-code AI Coding Assistant CLI Implementation Plan

## 1. Project Architecture

### 1.1 High-Level Architecture


[User Terminal] <-> [miden-code CLI] <-> [Sonnet 3.7 API]


                       |

                       v

           [Filesystem & Shell Access]


 ### 1.2 Core Components

 - **CLI Interface**: Handles user input/output and manages the chat session

 - **LLM Client**: Communicates with Sonnet 3.7 API via OpenRouter API

 - **File System Manager**: Provides safe abstraction for file operations

 - **Shell Command Executor**: Manages shell command execution with user permissions

 - **Chat Context Manager**: Maintains conversation history and context



 ## 2. Project Setup



 ### 2.1 Initial Repository Structure

miden-code/

├── Cargo.toml

├── src/

│   ├── main.rs              # Entry point

│   ├── cli.rs               # CLI interface and user interaction

│   ├── llm/                 # LLM integration

│   │   ├── mod.rs

│   │   ├── client.rs        # Sonnet API client

│   │   └── prompt.rs        # Prompt engineering

│   ├── tools/               # LLM filesystem and shell tools

│   │   ├── mod.rs

│   │   ├── filesystem.rs    # File operations

│   │   └── shell.rs         # Shell command execution

│   └── chat/                # Chat functionality

│       ├── mod.rs

│       ├── context.rs       # Conversation context

│       └── ui.rs            # Terminal UI

├── tests/                   # Integration tests

└── .github/                 # CI/CD configuration




 ### 2.2 Dependencies

 - **Core**:

   - `clap`: Command-line argument parsing

   - `tokio`: Async runtime

   - `reqwest`: HTTP client for API calls

   - `serde`: Serialization/deserialization

   - `serde_json`: JSON handling

   - `anyhow`: Error handling

 - **Terminal UI**:

   - `crossterm` or `termion`: Terminal manipulation

   - `ratatui`: TUI framework

 - **Other**:

   - `dirs`: User directory management

   - `syntect`: Syntax highlighting

   - `log`/`env_logger`: Logging

   - `dotenv`: Environment configuration



 ## 3. Implementation Phases



 ### Phase 1: Core Infrastructure (Week 1)

 - Set up project structure with Cargo

 - Implement basic CLI structure with clap

 - Create Sonnet 3.7 API client structure using the OpenRouter API

 - Implement simple prompt templating system

 - Set up basic terminal UI for text input/output



 ### Phase 2: Basic Chat Functionality (Week 2)

 - Implement chat context management

 - Develop basic conversation flow

 - Connect to Sonnet 3.7 API

 - Implement message history and context window management

 - Build initial terminal UI for conversation display



 ### Phase 3: File System Operations (Week 3)

 - Implement read file capability

 - Implement directory listing

 - Create proper permission checks and user confirmation flows

 - Implement file content display with syntax highlighting

 - Design and implement the tool calling format for the LLM



 ### Phase 4: Shell Command Execution (Week 4)

 - Design safe shell command execution mechanism

 - Implement command preview and confirmation

 - Capture and format command output

 - Add output parsing and display

 - Implement shell command suggestions



 ### Phase 5: Advanced Features and Testing (Week 5)

 - Implement session persistence

 - Add configuration management

 - Create comprehensive test suite

 - Optimize performance

 - Refine error handling and user experience



 ## 4. Implementation Details



 ### 4.1 LLM Tool Interfaces



 #### File Reading Tool


fn read_file(path: &str) -> Result<String, Error> {


 // 1. Validate file path

 // 2. Request user permission

 // 3. Read file content

 // 4. Return content or error


}




 #### Directory Listing Tool



fn list_directory(path: &str) -> Result<Vec, Error> {


 // 1. Validate directory path

 // 2. Request user permission

 // 3. Execute ls or equivalent

 // 4. Parse and return results


}




 #### Shell Command Execution Tool


fn execute_command(command: &str) -> Result<CommandOutput, Error> {


 // 1. Parse and validate command

 // 2. Display command to user

 // 3. Request explicit permission

 // 4. Execute command in controlled environment

 // 5. Capture output and return


}




 ### 4.2 Chat Context Management

 - Maintain conversation history with fixed-size context window

 - Implement context pruning strategies

 - Associate file operations with conversation turns



 ### 4.3 Terminal UI Design

 - Split-pane interface with chat history and input area

 - Status bar for system information

 - Syntax highlighting for code snippets

 - Progress indicators for API calls



 ## 5. Testing Strategy



 ### 5.1 Unit Tests

 - Test each component in isolation

 - Mock LLM API for predictable responses

 - Test file system operations with test directories



 ### 5.2 Integration Tests

 - End-to-end testing of complete conversation flows

 - Test tool invocations with controlled environments

 - Validate context retention and management



 ## 6. Security Considerations

 - Always require explicit user permission for file/shell operations

 - Implement safe command parsing to prevent injection attacks

 - Sanitize input/output to prevent terminal escape sequence attacks

 - Implement strict API key management

