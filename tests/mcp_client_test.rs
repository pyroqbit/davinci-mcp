#[cfg(test)]
mod tests {
    use std::io::{self, BufRead, BufReader, Write};
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;
    use serde_json::{json, Value};

    #[tokio::test]
    #[ignore] // Игнорируем по умолчанию, так как требует запуск сервера
    async fn test_mcp_server_protocol() {
        println!("🎬 MCP Server Protocol Integration Test");
        println!("{}", "=".repeat(50));

        // Start the MCP server
        let mut server = Command::new("cargo")
            .args(&["run", "--bin", "davinci-mcp-server"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start MCP server");

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
                    "name": "rust-integration-test-client",
                    "version": "1.0"
                }
            }
        });
        
        send_request(&mut stdin, &init_request).expect("Failed to send init request");
        let response = read_response(&mut reader).expect("Failed to read init response");
        assert!(response.contains("result"), "Initialize response should contain result");
        println!("📥 Response: {}", response);

        // Test 2: List tools
        println!("\n🛠️ Test 2: List available tools");
        let tools_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        });
        
        send_request(&mut stdin, &tools_request).expect("Failed to send tools request");
        let response = read_response(&mut reader).expect("Failed to read tools response");
        assert!(response.contains("tools"), "Tools response should contain tools list");
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
        
        send_request(&mut stdin, &switch_request).expect("Failed to send switch request");
        let response = read_response(&mut reader).expect("Failed to read switch response");
        assert!(response.contains("result"), "Switch response should contain result");
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
                    "name": "Rust MCP Integration Test Timeline",
                    "frame_rate": "24",
                    "resolution_width": 1920,
                    "resolution_height": 1080
                }
            }
        });
        
        send_request(&mut stdin, &timeline_request).expect("Failed to send timeline request");
        let response = read_response(&mut reader).expect("Failed to read timeline response");
        assert!(response.contains("result"), "Timeline response should contain result");
        println!("📥 Response: {}", response);

        println!("\n✅ All MCP protocol integration tests completed!");

        // Clean up
        server.kill().expect("Failed to kill server");
        println!("🔚 Server stopped");
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
} 