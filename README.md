# Log Server

A stateless HTTP server for managing log files on the local file system.

## Quick Start

```bash
cargo run
```

## API Documentation

- `GET /` - Minimal HTML welcome page
- `GET /hello` - Returns "Hello, World!"
- `GET /health` - Health check returning "OK"

## Configuration

- Set log level: `RUST_LOG=DEBUG cargo run`
- Default: `INFO` level

## Testing

```bash
cargo test          # All tests
cargo test -- --nocapture  # Verbose output
```

## Deployment

Compatible with Linux (Raspberry Pi) and Windows.

## Future Features

- File upload for log file processing
- Configurable paths for log management
- UI for browsing log files