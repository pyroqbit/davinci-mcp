# DaVinci Resolve MCP Server (Rust)

A high-performance Model Context Protocol (MCP) server for DaVinci Resolve automation, written in Rust.

## 🎉 Latest Updates (December 2024)

- **✅ All Tests Passing**: 100% test coverage with 43 comprehensive tests
- **🚀 120+ Professional Tools**: Complete automation suite for DaVinci Resolve
- **🔧 Phase 3 API Complete**: All missing API methods implemented
- **⚡ Enhanced Performance**: Optimized bridge implementation with simulation mode
- **🛠️ Code Quality**: Formatted, linted, and production-ready

## Features

- **120+ Professional Tools**: Complete automation suite for DaVinci Resolve
- **Pure Rust Implementation**: Fast, memory-safe, and reliable
- **Complete DaVinci Resolve API Coverage**: All major operations supported
- **Keyframe Animation System**: Professional-grade animation control
- **Timeline Item Manipulation**: Comprehensive timeline item property control
- **Color Grading Operations**: Advanced color correction and grading tools
- **Async/Await Support**: Non-blocking operations for better performance
- **Type Safety**: Compile-time guarantees for API correctness
- **Comprehensive Error Handling**: Detailed error reporting and recovery
- **Dual Mode Operation**: Real DaVinci Resolve connection + Simulation mode for testing

## Tool Categories (120+ Total)

### 🎬 Project & Timeline Management (15 tools)
- Project creation, opening, and page switching
- Timeline creation, deletion, and management
- Marker addition and timeline track operations
- Project properties and timeline format control

### 📁 Media Pool Operations (15 tools) 
- Media import and bin management
- Audio sync and clip operations
- Proxy media and subclip creation
- Advanced media pool folder operations

### 🎨 Color Grading Operations (12 tools)
- LUT application and export
- Color wheel parameter control
- Node management and grade copying
- Color preset save/apply/delete operations
- PowerGrade LUT batch export

### ⚡ Timeline Item Manipulation (10 tools)
- Transform properties (Pan, Tilt, Zoom, Rotation, etc.)
- Crop settings (Left, Right, Top, Bottom)
- Composite modes and opacity control
- Retiming and stabilization settings
- Audio properties (Volume, Pan, EQ)

### 🎞️ Keyframe Animation System (8 tools)
- Add, modify, and delete keyframes
- Interpolation control (Linear, Bezier, Ease-In/Out)
- Keyframe mode activation (All, Color, Sizing)
- Comprehensive keyframe inspection and management

### 🚀 Rendering & Delivery Operations (8 tools)
- Render queue management (add, start, clear)
- Real-time render status monitoring
- Project export with media packaging
- Custom render preset creation
- Professional delivery workflows

### ☁️ Cloud Operations (8 tools)
- Cloud project creation and management
- Project import/export to cloud
- User permission management
- Collaborative workflow support

### 🎛️ Cache & Optimization Operations (8 tools)
- Cache mode control (auto/on/off)
- Optimized media generation and deletion
- Proxy mode and quality settings
- Cache path configuration (local/network)

### 🖥️ Layout & Interface Management (6 tools)
- UI layout preset save/load/export/import
- Interface customization and automation
- Workspace management

### 🔧 Application Control (5 tools)
- Application quit and restart
- Settings and preferences dialogs
- System-level DaVinci Resolve control

### 🔍 Object Inspection & API Tools (4 tools)
- DaVinci Resolve API object help
- Custom object path inspection
- Advanced API debugging tools

### 🎵 Audio Operations (8 tools)
- Audio track management and naming
- Audio transcription and clearing
- Advanced audio workflow automation

### 🎭 Fusion Operations (4 tools)
- Fusion tool management
- Advanced compositing operations

### 📊 Gallery Operations (3 tools)
- Gallery still album management
- Advanced gallery workflow automation

### 🔗 Advanced Project Operations (15 tools)
- Project timeline management by index
- Current timeline operations
- Project naming and unique ID management
- Render job list and rendering control
- Project preset management
- Render format and codec control
- Render mode management
- Color groups management

## Performance & Quality

- **Tests**: 43 comprehensive tests (100% passing)
  - 12 Phase 3 API coverage tests
  - 17 integration tests  
  - 6 unit tests
  - 8 additional specialized tests
- **Performance**: O(log n) keyframe operations with binary search optimization
- **Memory**: Optimized bridge implementation with efficient state management
- **Architecture**: Professional-grade Rust implementation with Arc<Mutex> concurrency
- **Code Quality**: Formatted with rustfmt, linted with clippy, production-ready

## Test Coverage Summary

```
✅ test_phase3_api_coverage_summary
✅ test_phase3_gallery_api_coverage  
✅ test_phase3_mediapool_api_coverage
✅ test_phase3_comprehensive_performance
✅ test_phase3_fairlight_api_coverage
✅ test_phase3_mediapoolitem_api_coverage
✅ test_phase3_fusion_api_coverage
✅ test_tool_validation
✅ test_phase3_project_api_coverage
✅ test_single_tool_debug
✅ test_performance_benchmark
✅ test_comprehensive_tool_coverage
```

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

**Non-Commercial License** - Free for non-commercial use only.

This software is licensed under a custom non-commercial license that permits:
- ✅ Personal use
- ✅ Educational use  
- ✅ Research projects
- ✅ Open-source development
- ❌ Commercial use (requires separate commercial license)

See the [LICENSE](LICENSE) file for complete terms and conditions.

For commercial licensing inquiries, please contact the project maintainers.

## Architecture

This server implements the Model Context Protocol to provide seamless integration between AI assistants and DaVinci Resolve. The Rust implementation ensures:

- **Performance**: Native code execution with minimal overhead
- **Safety**: Memory safety and thread safety guaranteed by Rust
- **Reliability**: Comprehensive error handling and recovery
- **Maintainability**: Clean architecture with clear separation of concerns

The server communicates with DaVinci Resolve through its Python scripting API via a carefully designed bridge layer, providing type-safe access to all DaVinci Resolve functionality.
