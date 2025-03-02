# CLAUDE.md - Guidelines for miden-code

## Plan
- Implement the miden-code using the PRD in `prd.md` by going through the `execution_plan.md` and logging your progress in `progress.md` after every code change.

## Build/Lint/Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test specific: `cargo test test_name`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

## Code Style Guidelines
- **Formatting**: Follow Rust standard formatting with `cargo fmt`
- **Imports**: Group imports by crate, then alphabetically
- **Types**: Use strong typing; avoid `Option<T>` unwrapping without checks
- **Naming**: snake_case for variables/functions, CamelCase for types/structs
- **Error Handling**: Use `anyhow` for general errors, custom error types for specific cases
- **Documentation**: Document all public interfaces with rustdoc comments
- **Tools**: Implement tools with explicit permission checks and user confirmation
- **Security**: Sanitize all inputs, validate paths, prevent injection attacks

## Project Structure
- CLI interface in `src/cli.rs`
- LLM integration in `src/llm/`
- Tool implementations in `src/tools/`
- Chat functionality in `src/chat/`
