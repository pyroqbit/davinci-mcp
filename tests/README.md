# 🧪 Tests Directory

This directory contains all integration and unit tests for the DaVinci Resolve MCP Server (Rust Edition).

## 📁 Test Structure

### Integration Tests
- **`integration_test.rs`** - Comprehensive integration tests for all MCP server functionality
- **`simple_integration_test.rs`** - Basic integration tests for core operations
- **`real_connection_test.rs`** - Tests with real DaVinci Resolve connection (requires running DaVinci Resolve)
- **`native_integration_test.rs`** - Native FFI integration tests (requires DaVinci Resolve native libraries)
- **`mcp_client_test.rs`** - MCP protocol communication tests (requires server startup)

### Unit Tests
- **`unit_test.rs`** - Unit tests for individual components and error handling

## 🚀 Running Tests

### Run All Tests (excluding ignored)
```bash
cargo test
```

### Run Specific Test File
```bash
cargo test --test integration_test
cargo test --test simple_integration_test
```

### Run Tests Requiring DaVinci Resolve
```bash
# Make sure DaVinci Resolve is running with external scripting enabled
cargo test --test real_connection_test -- --ignored

# Test native FFI integration (requires DaVinci Resolve native libraries)
cargo test --test native_integration_test -- --ignored
```

### Run MCP Protocol Tests
```bash
# These tests start the MCP server automatically
cargo test --test mcp_client_test -- --ignored
```

### Run with Output
```bash
cargo test -- --nocapture
```

## 🔧 Test Categories

### ✅ Always Available
- `integration_test.rs` - Simulation mode tests
- `simple_integration_test.rs` - Basic functionality tests
- `unit_test.rs` - Unit tests

### ⚠️ Requires DaVinci Resolve
- `real_connection_test.rs` - Real DaVinci Resolve integration
  - Requires DaVinci Resolve 20+ running
  - Requires "External scripting using local network" enabled in preferences
- `native_integration_test.rs` - Native FFI integration
  - Requires DaVinci Resolve native libraries installed
  - Tests low-level FFI bindings

### 🖥️ Requires Server Startup
- `mcp_client_test.rs` - Full MCP protocol testing
  - Automatically starts and stops the MCP server
  - Tests complete MCP communication flow

## 📊 Test Coverage

The tests cover:
- ✅ Project management operations
- ✅ Timeline operations and manipulation
- ✅ Media pool management
- ✅ Color grading and LUT operations
- ✅ Timeline item transformations
- ✅ Keyframe animation
- ✅ Render and delivery operations
- ✅ MCP protocol compliance
- ✅ Error handling and validation
- ✅ Real DaVinci Resolve integration

## 🎯 Test Philosophy

- **Unit Tests**: Test individual functions and error conditions
- **Integration Tests**: Test complete workflows in simulation mode
- **Real Connection Tests**: Verify actual DaVinci Resolve integration
- **Protocol Tests**: Ensure MCP specification compliance

All tests are designed to be fast, reliable, and provide clear feedback on functionality. 