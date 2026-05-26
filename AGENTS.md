## Testing Policy

### Rule 1: Update Tests on Every Code Change
For every code change, tests must be updated accordingly.

### Unit Tests (same directory as code)
Location: `src/main.rs` within `#[cfg(test)]` module
- Test handlers in isolation
- Mock dependencies with mockall
- Test error paths

### Before merging any change:
1. Run `cargo test --all`
2. Ensure all tests pass
3. Update failing tests

### Mockall Usage
For stateless handlers, minimal mocking needed, but use mockall for future dependencies.

### Use powershell commands
Use powershell commands and don't use bash commands.