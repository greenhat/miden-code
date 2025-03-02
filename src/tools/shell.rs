use anyhow::{anyhow, Result};
use std::process::Command;

/// Output from executing a shell command
#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

/// Execute a shell command and return its output
pub async fn execute_command(command: &str) -> Result<CommandOutput> {
    // 1. Parse and validate command
    if command.trim().is_empty() {
        return Err(anyhow!("Empty command"));
    }

    // Check for potentially dangerous commands
    let dangerous_commands = ["rm -rf", "sudo", "> /dev/", "mkfs"];
    for dc in dangerous_commands {
        if command.contains(dc) {
            return Err(anyhow!("Potentially dangerous command detected: {}", dc));
        }
    }

    // 2. In a real implementation, we would display the command to the user and request permission

    // 3. Execute the command in a controlled environment
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    // 4. Capture and return the output
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    Ok(CommandOutput {
        stdout,
        stderr,
        success,
    })
}