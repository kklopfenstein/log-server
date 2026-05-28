# Log Server

A stateless HTTP server for managing log files on the local file system.

## Quick Start

### Windows

```powershell
$env:CONFIG_FILE = ".\example\config.yaml"
cargo run
```

### Linux

```bash
CONFIG_FILE="example/config.yaml" cargo run
```

## API Documentation

- `GET /` - Minimal HTML welcome page
- `GET /hello` - Returns "Hello, World!"
- `GET /health` - Health check returning "OK"
- `GET /logs/{file}` - Return logs for a file.

## Configuration

- Set log level: `RUST_LOG=DEBUG cargo run`
- Default: `INFO` level

## Testing

```bash
cargo test          # All tests
cargo test -- --nocapture  # Verbose output
```

## Deployment

Compatible with Linux and Windows.

## Future Features

- File upload for log file processing
- Configurable paths for log management
- UI for browsing log files