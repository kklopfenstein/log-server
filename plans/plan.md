# Log Server - /logs Endpoint Implementation Plan

## Overview
Add a `/logs/${name}` endpoint to serve paginated log file content via YAML config file.

## Configuration

### CLI Argument
- `--config <path>`: Path to YAML config file (required)

### YAML Format
```yaml
rclone-log: /var/log/rclone-log.log
app-log: C:\Users\kklop\somelog.log
nginx-error: /var/log/nginx/error.log
```

### API Endpoint
- Path: `/logs/{name}`
- Query params: `cursor` (line from bottom, 0 = EOF), `limit` (count)
- Response: JSON lines array in reverse chronological order

## Implementation Changes

### 1. Cargo.toml
- Add `serde_yaml`, `serde` dependencies

### 2. src/config.rs (new)
- `LogConfig` struct with files HashMap
- `load()` method to parse YAML and validate file paths
- Getter for file paths

### 3. src/handlers.rs
- Add `/logs/${name}` handler
- Route `/logs/{name}` to query config for file path
- Read paginated content from file
- Return JSON with line content and line numbers

### 4. src/main.rs
- Add `--config` CLI argument parsing
- Load config at startup
- Pass config to router

## Error Handling

- 400: Missing `/config` argument or invalid CLI args
- 404: Log file not found in config
- 400: Invalid cursor/limit parameters
- 500: File read errors, IO errors

## Tests

- Test valid pagination scenarios
- Test cursor at 0 (EOF)
- Test limit exceeding file size
- Test missing config file (404)
- Test corrupted YAML
- Test cursor/limit boundary conditions