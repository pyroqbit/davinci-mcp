#!/usr/bin/env python3
import json
import subprocess
import sys

def test_tools_list():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Send two messages at once to see what happens
    messages = [
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}
    ]
    
    for msg in messages:
        print(f"SEND: {json.dumps(msg)}")
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    # Read all responses
    for line in proc.stdout:
        if line.strip():
            print(f"RECV: {line.strip()}")
    
    # Check for errors
    errors = proc.stderr.read()
    if errors:
        print(f"ERRORS: {errors}")
    
    proc.wait()

if __name__ == "__main__":
    test_tools_list() 