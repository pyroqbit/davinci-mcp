# DaVinci Resolve MCP Server (Rust)

A high-performance Model Context Protocol (MCP) server for DaVinci Resolve automation, written in Rust.

## Features

- **80+ Professional Tools**: Complete automation suite for DaVinci Resolve
- **Pure Rust Implementation**: Fast, memory-safe, and reliable
- **Complete DaVinci Resolve API Coverage**: All major operations supported
- **Keyframe Animation System**: Professional-grade animation control (Phase 4 Week 2)
- **Timeline Item Manipulation**: Comprehensive timeline item property control
- **Color Grading Operations**: Advanced color correction and grading tools
- **Async/Await Support**: Non-blocking operations for better performance
- **Type Safety**: Compile-time guarantees for API correctness
- **Comprehensive Error Handling**: Detailed error reporting and recovery

## Tool Categories (80+ Total)

### Extended Project Management (5 tools) ✨ NEW
- Media clip deletion and bin organization
- Folder export to DRB format
- Batch audio transcription for folders
- Advanced media pool management

### Cache & Optimization Operations (7 tools) ✨ NEW
- Cache mode control (auto/on/off)
- Optimized media generation and deletion
- Proxy mode and quality settings
- Cache path configuration (local/network)

### Extended Color Operations (3 tools) ✨ NEW
- Color preset album management
- PowerGrade LUT batch export
- Advanced color workflow automation

### Layout & Interface Management (5 tools) ✨ NEW
- UI layout preset save/load/export/import
- Interface customization and automation
- Workspace management

### Application Control (4 tools) ✨ NEW
- Application quit and restart
- Settings and preferences dialogs
- System-level DaVinci Resolve control

### Cloud Operations (6 tools) ✨ NEW
- Cloud project creation and management
- Project import/export to cloud
- User permission management
- Collaborative workflow support

### Object Inspection (2 tools) ✨ NEW
- DaVinci Resolve API object help
- Custom object path inspection

### Project Properties (2 tools) ✨ NEW
- Project property configuration
- Timeline format settings

### Project & Timeline Management (11 tools)
- Project creation, opening, and page switching
- Timeline creation, deletion, and management
- Marker addition and timeline track operations

### Media Pool Operations (10 tools) 
- Media import and bin management
- Audio sync and clip operations
- Proxy media and subclip creation

### Color Grading Operations (8 tools)
- LUT application and export
- Color wheel parameter control
- Node management and grade copying
- Color preset save/apply/delete operations

### Timeline Item Manipulation (8 tools)
- Transform properties (Pan, Tilt, Zoom, Rotation, etc.)
- Crop settings (Left, Right, Top, Bottom)
- Composite modes and opacity control
- Retiming and stabilization settings
- Audio properties (Volume, Pan, EQ)

### Keyframe Animation System (6 tools)
- Add, modify, and delete keyframes
- Interpolation control (Linear, Bezier, Ease-In/Out, Hold)
- Keyframe mode activation (All, Color, Sizing)
- Comprehensive keyframe inspection and management

### Rendering & Delivery Operations (6 tools) ✨ NEW
- Render queue management (add, start, clear)
- Real-time render status monitoring
- Project export with media packaging
- Custom render preset creation
- Professional delivery workflows

## Performance & Quality

- **Tests**: 23 comprehensive tests (17 integration + 6 unit)
- **Performance**: O(log n) keyframe operations with binary search optimization
- **Memory**: Optimized 64-byte keyframe representation
- **Architecture**: Professional-grade Rust implementation with Arc<Mutex> concurrency

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

📚 **[Complete Documentation](docs/README.md)** - Comprehensive documentation index

### Quick Links
- **[Quick Start Guide](docs/guides/QUICK_START.md)** - Get up and running in 5 minutes
- **[Usage Guide](docs/guides/USAGE_GUIDE.md)** - Detailed usage instructions and examples
- **[Features Overview](docs/FEATURES.md)** - Complete feature list and capabilities
- **[Project Reports](docs/reports/)** - Development progress and achievement reports

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
