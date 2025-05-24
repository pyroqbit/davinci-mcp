#!/usr/bin/env python3
"""
Complete MCP protocol test for the DaVinci Resolve MCP server.
"""

import json
import subprocess
import sys
import time

def test_complete_mcp_protocol():
    """Test the MCP server with complete protocol sequence."""
    
    server_path = "./target/release/davinci-mcp-server"
    
    try:
        proc = subprocess.Popen(
            [server_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        def send_message(message):
            print(f"→ {json.dumps(message)}")
            proc.stdin.write(json.dumps(message) + "\n")
            proc.stdin.flush()
        
        def read_response():
            response_line = proc.stdout.readline().strip()
            if response_line:
                print(f"← {response_line}")
                return json.loads(response_line)
            return None
        
        # Step 1: Initialize
        print("=== STEP 1: Initialize ===")
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }
        }
        
        send_message(init_request)
        init_response = read_response()
        
        if not init_response or "result" not in init_response:
            print("❌ Initialization failed")
            return False
        
        print("✅ Initialization successful!")
        
        # Step 2: Send initialized notification
        print("\n=== STEP 2: Initialized Notification ===")
        initialized_notification = {
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }
        
        send_message(initialized_notification)
        print("✅ Initialized notification sent")
        
        # Step 3: List tools
        print("\n=== STEP 3: List Tools ===")
        tools_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }
        
        send_message(tools_request)
        tools_response = read_response()
        
        if tools_response and "result" in tools_response and "tools" in tools_response["result"]:
            tools = tools_response["result"]["tools"]
            print(f"✅ Found {len(tools)} tools:")
            for i, tool in enumerate(tools, 1):
                print(f"  {i:2d}. {tool['name']:20s} - {tool['description']}")
        else:
            print("❌ No tools found in response")
            return False
        
        # Step 4: Test a tool call
        print("\n=== STEP 4: Test Tool Call ===")
        tool_call_request = {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "create_project",
                "arguments": {
                    "name": "Test Project from Rust MCP"
                }
            }
        }
        
        send_message(tool_call_request)
        tool_response = read_response()
        
        if tool_response and "result" in tool_response:
            content = tool_response["result"]["content"][0]["text"]
            print(f"✅ Tool call successful: {content}")
        else:
            print("❌ Tool call failed")
            return False
        
        # Step 5: Test another tool call
        print("\n=== STEP 5: Test Media Import ===")
        import_request = {
            "jsonrpc": "2.0",
            "id": 4,
            "method": "tools/call",
            "params": {
                "name": "import_media",
                "arguments": {
                    "file_path": "/path/to/test_video.mp4"
                }
            }
        }
        
        send_message(import_request)
        import_response = read_response()
        
        if import_response and "result" in import_response:
            content = import_response["result"]["content"][0]["text"]
            print(f"✅ Media import successful: {content}")
        else:
            print("❌ Media import failed")
            return False
        
        print("\n🎉 All tests passed! Pure Rust MCP server is working perfectly!")
        
        # Clean up
        proc.terminate()
        proc.wait(timeout=2)
        
        return True
        
    except FileNotFoundError:
        print(f"❌ Server binary not found: {server_path}")
        print("Please run: cargo build --release")
        return False
    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return False

def check_binary_info():
    """Check information about the compiled binary."""
    import os
    
    binary_path = "./target/release/davinci-mcp-server"
    if os.path.exists(binary_path):
        stat = os.stat(binary_path)
        size_mb = stat.st_size / (1024 * 1024)
        print(f"📁 Binary size: {size_mb:.2f} MB")
        print(f"📅 Modified: {time.ctime(stat.st_mtime)}")
    else:
        print("❌ Binary not found")

if __name__ == "__main__":
    print("🚀 DaVinci Resolve MCP Server (Pure Rust) - Complete Test")
    print("=" * 65)
    
    check_binary_info()
    print()
    
    success = test_complete_mcp_protocol()
    sys.exit(0 if success else 1) 