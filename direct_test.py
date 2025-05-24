#!/usr/bin/env python3
import json
import subprocess
import sys

def direct_test():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Test sequence
    messages = [
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}},
        {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_project", "arguments": {"name": "Test"}}},
        {"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_timelines_tool", "arguments": {}}}
    ]
    
    print("=== SENDING MESSAGES ===")
    for i, msg in enumerate(messages):
        print(f"Message {i+1}: {json.dumps(msg)}")
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    print("\n=== RECEIVING RESPONSES ===")
    response_count = 0
    for line in proc.stdout:
        if line.strip():
            response_count += 1
            print(f"Response {response_count}: {line.strip()}")
            
            try:
                response = json.loads(line.strip())
                if response.get("id") == 2:  # Tools list
                    tools = response.get("result", {}).get("tools", [])
                    print(f"  → Found {len(tools)} tools")
                elif response.get("id") == 4:  # list_timelines_tool
                    content = response.get("result", {}).get("content", [{}])[0].get("text", "")
                    print(f"  → Timeline tool result: {content}")
            except:
                pass
    
    errors = proc.stderr.read()
    if errors.strip():
        print(f"\n=== STDERR ===")
        print(errors)
        
    proc.wait()
    print(f"\nTotal responses: {response_count}")

if __name__ == "__main__":
    direct_test() 