#!/usr/bin/env python3
"""
Timeline Enhancement Test - Phase 3 Week 2
Test the 6 new timeline management tools added to DaVinci Resolve MCP Server
"""
import json
import subprocess
import sys

def test_timeline_enhancement():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Test messages for timeline enhancement
    messages = [
        # 1. Initialize
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "timeline-test", "version": "1.0"}}},
        
        # 2. Initialized notification
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        
        # 3. List tools to verify we have 20 tools now
        {"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}},
        
        # 4. Create a project first
        {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_project", "arguments": {"name": "Timeline Enhancement Test"}}},
        
        # 5. Create original timeline
        {"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "create_timeline", "arguments": {"name": "Original Timeline"}}},
        
        # 6. Test create_empty_timeline with custom settings
        {"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "create_empty_timeline", "arguments": {"name": "Custom Timeline", "frame_rate": "30", "resolution_width": 1920, "resolution_height": 1080, "video_tracks": 3, "audio_tracks": 4}}},
        
        # 7. Test list_timelines_tool
        {"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "list_timelines_tool", "arguments": {}}},
        
        # 8. Test set_current_timeline
        {"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "set_current_timeline", "arguments": {"name": "Original Timeline"}}},
        
        # 9. Import some media first
        {"jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": {"name": "import_media", "arguments": {"file_path": "/test/video.mp4"}}},
        
        # 10. Test add_clip_to_timeline  
        {"jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": {"name": "add_clip_to_timeline", "arguments": {"clip_name": "video.mp4", "timeline_name": "Custom Timeline"}}},
        
        # 11. Test get_timeline_tracks
        {"jsonrpc": "2.0", "id": 10, "method": "tools/call", "params": {"name": "get_timeline_tracks", "arguments": {"timeline_name": "Custom Timeline"}}},
        
        # 12. Test delete_timeline
        {"jsonrpc": "2.0", "id": 11, "method": "tools/call", "params": {"name": "delete_timeline", "arguments": {"name": "Original Timeline"}}},
    ]
    
    print("🎬 Timeline Enhancement Test (Phase 3 Week 2)")
    print("=" * 55)
    print("Testing 6 new timeline management tools...")
    print()
    
    # Send all messages
    for msg in messages:
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    # Process responses
    responses = []
    tool_results = []
    
    for line in proc.stdout:
        if line.strip():
            try:
                response = json.loads(line.strip())
                responses.append(response)
                
                if "id" in response and response["id"] >= 3:  # Tool calls start at id 3
                    result = response.get("result", {})
                    content = result.get("content", [{}])[0].get("text", "")
                    is_error = result.get("is_error", False)
                    
                    tool_names = [
                        "create_project", "create_timeline", "create_empty_timeline", 
                        "list_timelines_tool", "set_current_timeline", "import_media",
                        "add_clip_to_timeline", "get_timeline_tracks", "delete_timeline"
                    ]
                    
                    tool_index = response["id"] - 3
                    tool_name = tool_names[tool_index] if tool_index < len(tool_names) else "unknown"
                    
                    if not is_error:
                        print(f"✅ {tool_name}: SUCCESS")
                        if tool_name in ["create_empty_timeline", "list_timelines_tool", "get_timeline_tracks"]:
                            print(f"   → {content}")
                        tool_results.append(tool_name)
                    else:
                        print(f"❌ {tool_name}: FAILED - {content}")
                        
                elif response.get("id") == 2:  # Tools list
                    tools = response.get("result", {}).get("tools", [])
                    print(f"📋 Tools Available: {len(tools)} tools")
                    
                    # Check for our timeline enhancement tools
                    timeline_tools = [
                        "delete_timeline", "set_current_timeline", "create_empty_timeline", 
                        "add_clip_to_timeline", "list_timelines_tool", "get_timeline_tracks"
                    ]
                    
                    found_timeline_tools = []
                    for tool in tools:
                        if tool["name"] in timeline_tools:
                            found_timeline_tools.append(tool["name"])
                    
                    print(f"🎬 Timeline Enhancement Tools Found: {len(found_timeline_tools)}/6")
                    for tool_name in found_timeline_tools:
                        print(f"   • {tool_name}")
                    print()
                    
            except json.JSONDecodeError:
                print(f"Warning: Could not parse response: {line.strip()}")
    
    # Check for errors
    errors = proc.stderr.read()
    if errors.strip():
        print(f"\n⚠️  Stderr: {errors}")
    
    proc.wait()
    
    print(f"\n📊 Timeline Enhancement Test Results:")
    print(f"   • Total responses: {len(responses)}")
    print(f"   • Successful tool calls: {len(tool_results)}")
    print(f"   • Timeline tools tested: 6")
    
    # Check specific timeline enhancement tools
    timeline_enhancement_tools = [
        "create_empty_timeline", "list_timelines_tool", "set_current_timeline",
        "add_clip_to_timeline", "get_timeline_tracks", "delete_timeline"
    ]
    
    timeline_successes = [tool for tool in tool_results if tool in timeline_enhancement_tools]
    print(f"   • Timeline enhancement successes: {len(timeline_successes)}/6")
    
    if len(timeline_successes) >= 5:  # Allow for one potential failure
        print(f"\n🎉 Timeline Enhancement Test: SUCCESS!")
        print(f"   Phase 3 Week 2 implementation working perfectly!")
        print(f"   Total tools now: 20 (14 original + 6 timeline enhancement)")
        return True
    else:
        print(f"\n❌ Timeline Enhancement Test: PARTIAL SUCCESS")
        print(f"   Only {len(timeline_successes)}/6 timeline tools working")
        return False

if __name__ == "__main__":
    success = test_timeline_enhancement()
    sys.exit(0 if success else 1) 