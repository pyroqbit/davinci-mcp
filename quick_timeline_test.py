#!/usr/bin/env python3
import json
import subprocess

def test_one_timeline_tool():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Simple test sequence
    messages = [
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "create_project", "arguments": {"name": "Test Project"}}},
        {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_empty_timeline", "arguments": {"name": "Test Timeline", "frame_rate": "24", "resolution_width": 1920, "resolution_height": 1080}}},
        {"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_timelines_tool", "arguments": {}}}
    ]
    
    print("🧪 Quick Timeline Enhancement Test")
    print("=" * 40)
    
    for msg in messages:
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    # Read responses
    response_count = 0
    for line in proc.stdout:
        if line.strip():
            try:
                response = json.loads(line.strip())
                response_count += 1
                
                if "id" in response:
                    if response["id"] == 2:
                        print("✅ create_project: SUCCESS")
                    elif response["id"] == 3:
                        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
                        print(f"✅ create_empty_timeline: {content}")
                    elif response["id"] == 4:
                        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
                        print(f"✅ list_timelines_tool: {content}")
                        
            except json.JSONDecodeError:
                print(f"Parse error: {line.strip()}")
    
    errors = proc.stderr.read()
    if errors.strip():
        print(f"Errors: {errors}")
        
    proc.wait()
    print(f"\nResponses received: {response_count}")

if __name__ == "__main__":
    test_one_timeline_tool() 