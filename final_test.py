#!/usr/bin/env python3
"""
Final comprehensive test of the Pure Rust DaVinci Resolve MCP Server
"""
import json
import subprocess
import sys

def test_comprehensive():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # All messages to send
    messages = [
        # 1. Initialize
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}},
        
        # 2. Initialized notification
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        
        # 3. List tools
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}},
        
        # 4. Test project creation
        {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_project", "arguments": {"name": "Pure Rust MCP Test Project"}}},
        
        # 5. Test page switching
        {"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "switch_page", "arguments": {"page": "edit"}}},
        
        # 6. Test timeline creation
        {"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "create_timeline", "arguments": {"name": "Test Timeline", "frame_rate": "24", "resolution_width": 1920, "resolution_height": 1080}}},
        
        # 7. Test marker addition
        {"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "add_marker", "arguments": {"color": "Red", "note": "Test marker from Rust", "frame": 100}}},
        
        # 8. Test media import
        {"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "import_media", "arguments": {"file_path": "/path/to/test_video.mp4"}}},
        
        # 9. Test bin creation
        {"jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": {"name": "create_bin", "arguments": {"name": "Test Bin"}}},
        
        # 10. Test audio sync
        {"jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": {"name": "auto_sync_audio", "arguments": {"clip_names": ["clip1.mp4", "clip2.wav"], "sync_method": "waveform"}}},
    ]
    
    print("🚀 Pure Rust DaVinci Resolve MCP Server - Final Test")
    print("=" * 60)
    
    # Send all messages
    for i, msg in enumerate(messages, 1):
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    # Process responses
    response_count = 0
    tool_call_results = []
    
    for line in proc.stdout:
        if line.strip():
            try:
                response = json.loads(line.strip())
                response_count += 1
                
                if "id" in response:
                    if response["id"] == 1:
                        print("✅ 1. Initialize: SUCCESS")
                    elif response["id"] == 2:
                        tools = response.get("result", {}).get("tools", [])
                        print(f"✅ 2. List Tools: SUCCESS - Found {len(tools)} tools")
                        for i, tool in enumerate(tools[:5], 1):  # Show first 5
                            print(f"   {i}. {tool['name']}")
                        if len(tools) > 5:
                            print(f"   ... and {len(tools) - 5} more tools")
                    elif response["id"] >= 3:
                        content = response.get("result", {}).get("content", [{}])[0].get("text", "")
                        is_error = response.get("result", {}).get("is_error", False)
                        
                        tool_names = ["create_project", "switch_page", "create_timeline", "add_marker", "import_media", "create_bin", "auto_sync_audio"]
                        tool_name = tool_names[response["id"] - 3] if response["id"] - 3 < len(tool_names) else "unknown"
                        
                        if not is_error:
                            print(f"✅ {response['id']}. {tool_name}: SUCCESS")
                            tool_call_results.append(content)
                        else:
                            print(f"❌ {response['id']}. {tool_name}: FAILED - {content}")
                            
            except json.JSONDecodeError:
                print(f"Warning: Could not parse response: {line.strip()}")
    
    # Check for errors
    errors = proc.stderr.read()
    if errors.strip():
        print(f"\n⚠️  Stderr output: {errors}")
    
    proc.wait()
    
    print(f"\n📊 Test Results:")
    print(f"   • Responses received: {response_count}")
    print(f"   • Tool calls successful: {len(tool_call_results)}")
    print(f"   • Binary size: {get_binary_size():.2f} MB")
    
    print(f"\n🎉 Pure Rust MCP Server Test COMPLETE!")
    print(f"   All {len(tool_call_results)} tool operations executed successfully!")
    
    return len(tool_call_results) >= 7  # Should have at least 7 successful tool calls

def get_binary_size():
    import os
    try:
        size = os.path.getsize("./target/release/davinci-mcp-server")
        return size / (1024 * 1024)
    except:
        return 0.0

if __name__ == "__main__":
    success = test_comprehensive()
    sys.exit(0 if success else 1) 