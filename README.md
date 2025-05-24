# DaVinci Resolve MCP Server (Rust)

A high-performance Model Context Protocol (MCP) server for DaVinci Resolve automation, written in Rust.

## Features

- **Pure Rust Implementation**: Fast, memory-safe, and reliable
- **Complete DaVinci Resolve API Coverage**: All major operations supported
- **Async/Await Support**: Non-blocking operations for better performance
- **Type Safety**: Compile-time guarantees for API correctness
- **Comprehensive Error Handling**: Detailed error reporting and recovery

## Project Structure

```
davinci-mcp-rs/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── server.rs           # Main MCP server implementation
│   ├── error.rs            # Error types and handling
│   ├── tools/              # MCP tool implementations
│   │   └── mod.rs          # Tool definitions and handlers
│   ├── bridge/             # DaVinci Resolve API bridge
│   ├── config/             # Configuration management
│   └── bin/                # Binary executables
├── tests/                  # Test suites
│   ├── integration_test.rs # Integration tests
│   └── unit_test.rs        # Unit tests
├── docs/                   # Documentation
│   ├── development/        # Development docs
│   ├── phases/             # Project phase documentation
│   ├── FEATURES.md         # Feature documentation
│   └── USAGE_GUIDE.md      # Usage guide
├── Cargo.toml              # Project configuration
└── README.md               # This file
```

## Installation

### Prerequisites

- Rust 1.70+ 
- DaVinci Resolve installed and running
- Python 3.8+ (for DaVinci Resolve's scripting API)

### Building

```bash
# Clone the repository
git clone https://github.com/modelcontextprotocol/davinci-mcp-rs
cd davinci-mcp-rs

# Build the project
cargo build --release

# Run tests
cargo test
```

## Usage

### Starting the Server

```bash
# Development mode
cargo run

# Production mode
cargo run --release
```

### Configuration

The server can be configured through environment variables or a configuration file. See `docs/USAGE_GUIDE.md` for detailed configuration options.

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test integration_test
cargo test unit_test

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check for security vulnerabilities
cargo audit
```

## Documentation

- [Features](docs/FEATURES.md) - Complete feature list
- [Usage Guide](docs/USAGE_GUIDE.md) - Detailed usage instructions
- [Development Status](docs/development/DEVELOPMENT_STATUS.md) - Current development status
- [Implementation Summary](docs/development/IMPLEMENTATION_SUMMARY.md) - Technical implementation details

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Architecture

This server implements the Model Context Protocol to provide seamless integration between AI assistants and DaVinci Resolve. The Rust implementation ensures:

- **Performance**: Native code execution with minimal overhead
- **Safety**: Memory safety and thread safety guaranteed by Rust
- **Reliability**: Comprehensive error handling and recovery
- **Maintainability**: Clean architecture with clear separation of concerns

The server communicates with DaVinci Resolve through its Python scripting API via a carefully designed bridge layer, providing type-safe access to all DaVinci Resolve functionality.
