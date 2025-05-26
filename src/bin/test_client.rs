use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Testing DaVinci Resolve MCP Server (Pure Rust)");
    println!("{}", "=".repeat(50));

    // Start the MCP server
    let mut server = Command::new("cargo")
        .args(&["run", "--bin", "davinci-mcp-server"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = server.stdin.take().unwrap();
    let stdout = server.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    // Give server time to start
    thread::sleep(Duration::from_secs(2));

    // Test 1: Initialize
    println!("\n🔧 Test 1: Initialize MCP connection");
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "rust-test-client",
                "version": "1.0"
            }
        }
    });
    
    send_request(&mut stdin, &init_request)?;
    let response = read_response(&mut reader)?;
    println!("📥 Response: {}", response);

    // Test 2: List tools
    println!("\n🛠️ Test 2: List available tools");
    let tools_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    });
    
    send_request(&mut stdin, &tools_request)?;
    let response = read_response(&mut reader)?;
    println!("📥 Response: {}", response);

    // Test 3: Switch to Edit page
    println!("\n📄 Test 3: Switch to Edit page");
    let switch_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "switch_page",
            "arguments": {
                "page": "edit"
            }
        }
    });
    
    send_request(&mut stdin, &switch_request)?;
    let response = read_response(&mut reader)?;
    println!("📥 Response: {}", response);

    // Test 4: Create timeline
    println!("\n📁 Test 4: Create a test timeline");
    let timeline_request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "create_empty_timeline",
            "arguments": {
                "name": "Rust MCP Test Timeline",
                "frame_rate": "24",
                "resolution_width": 1920,
                "resolution_height": 1080
            }
        }
    });
    
    send_request(&mut stdin, &timeline_request)?;
    let response = read_response(&mut reader)?;
    println!("📥 Response: {}", response);

    // Test 5: Add marker
    println!("\n🎯 Test 5: Add a marker to timeline");
    let marker_request = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "add_marker",
            "arguments": {
                "frame": 100,
                "color": "Red",
                "note": "Rust MCP Test Marker"
            }
        }
    });
    
    send_request(&mut stdin, &marker_request)?;
    let response = read_response(&mut reader)?;
    println!("📥 Response: {}", response);

    // Test 6: List timelines
    println!("\n📋 Test 6: List all timelines");
    let list_request = json!({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "tools/call",
        "params": {
            "name": "list_timelines_tool",
            "arguments": {
                "random_string": "test"
            }
        }
    });
    
    send_request(&mut stdin, &list_request)?;
    let response = read_response(&mut reader)?;
    println!("📥 Response: {}", response);

    println!("\n✅ All tests completed!");

    // Clean up
    server.kill()?;
    println!("🔚 Server stopped");

    Ok(())
}

fn send_request(stdin: &mut std::process::ChildStdin, request: &Value) -> io::Result<()> {
    let request_str = format!("{}\n", request.to_string());
    println!("📤 Sending: {}", request["method"].as_str().unwrap_or("unknown"));
    stdin.write_all(request_str.as_bytes())?;
    stdin.flush()?;
    Ok(())
}

fn read_response(reader: &mut BufReader<std::process::ChildStdout>) -> io::Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line.trim().to_string())
} 