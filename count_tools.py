#!/usr/bin/env python3
import json
import subprocess

def count_tools():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Send initialization and tools list request
    messages = [
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
    ]
    
    for msg in messages:
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    # Read responses
    for line in proc.stdout:
        if line.strip():
            try:
                response = json.loads(line.strip())
                if response.get("id") == 2:  # Tools list response
                    tools = response.get("result", {}).get("tools", [])
                    print(f"Total tools: {len(tools)}")
                    
                    # List timeline enhancement tools
                    timeline_tools = []
                    for tool in tools:
                        name = tool.get("name", "")
                        if any(keyword in name for keyword in ["timeline", "delete_timeline", "set_current", "create_empty", "add_clip_to", "list_timelines", "get_timeline"]):
                            timeline_tools.append(name)
                    
                    print(f"Timeline-related tools: {len(timeline_tools)}")
                    for tool in timeline_tools:
                        print(f"  • {tool}")
                    
                    return len(tools)
                    
            except json.JSONDecodeError:
                pass
    
    proc.wait()
    return 0

if __name__ == "__main__":
    count_tools() 