#!/usr/bin/env python3
"""
Simple test script for the DaVinci Resolve MCP server.
"""

import json
import subprocess
import sys

def test_mcp_server():
    """Test the MCP server with proper initialization sequence."""
    
    # Start the server process
    server_path = "./target/release/davinci-mcp-server"
    
    try:
        proc = subprocess.Popen(
            [server_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        # Step 1: Initialize
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
        
        print("Sending initialize request...")
        print(f"Request: {json.dumps(init_request)}")
        
        # Send initialize request
        proc.stdin.write(json.dumps(init_request) + "\n")
        proc.stdin.flush()
        
        # Read response
        response_line = proc.stdout.readline()
        print(f"Response: {response_line.strip()}")
        
        if response_line:
            try:
                init_response = json.loads(response_line)
                print(f"Parsed response: {json.dumps(init_response, indent=2)}")
                
                if "result" in init_response:
                    print("✅ Initialization successful!")
                    
                    # Step 2: List tools
                    tools_request = {
                        "jsonrpc": "2.0",
                        "id": 2,
                        "method": "tools/list",
                        "params": {}
                    }
                    
                    print("\nSending tools/list request...")
                    proc.stdin.write(json.dumps(tools_request) + "\n")
                    proc.stdin.flush()
                    
                    tools_response_line = proc.stdout.readline()
                    print(f"Tools response: {tools_response_line.strip()}")
                    
                    if tools_response_line:
                        tools_response = json.loads(tools_response_line)
                        if "result" in tools_response and "tools" in tools_response["result"]:
                            tools = tools_response["result"]["tools"]
                            print(f"\n✅ Found {len(tools)} tools:")
                            for i, tool in enumerate(tools, 1):
                                print(f"  {i}. {tool['name']} - {tool['description']}")
                        else:
                            print("❌ No tools found in response")
                    
                else:
                    print(f"❌ Initialization failed: {init_response}")
                    
            except json.JSONDecodeError as e:
                print(f"❌ Failed to parse response: {e}")
                print(f"Raw response: {response_line}")
        
        # Clean up
        proc.terminate()
        proc.wait(timeout=2)
        
    except FileNotFoundError:
        print(f"❌ Server binary not found: {server_path}")
        print("Please run: cargo build --release")
        return False
    except Exception as e:
        print(f"❌ Test failed: {e}")
        return False
    
    return True

if __name__ == "__main__":
    print("Testing DaVinci Resolve MCP Server...")
    print("=" * 50)
    success = test_mcp_server()
    sys.exit(0 if success else 1) 