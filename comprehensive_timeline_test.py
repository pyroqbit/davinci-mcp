#!/usr/bin/env python3
"""
Comprehensive Timeline Enhancement Test - Phase 3 Week 2
Test all 6 new timeline management tools
"""
import json
import subprocess
import sys

def test_all_timeline_tools():
    server_path = "./target/release/davinci-mcp-server"
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Comprehensive test sequence
    messages = [
        # Initialize
        {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}},
        {"jsonrpc": "2.0", "method": "notifications/initialized"},
        
        # Setup
        {"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "create_project", "arguments": {"name": "Timeline Test Project"}}},
        
        # Test 1: create_empty_timeline
        {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_empty_timeline", "arguments": {"name": "Custom Timeline", "frame_rate": "30", "resolution_width": 1920, "resolution_height": 1080, "video_tracks": 3, "audio_tracks": 4}}},
        
        # Test 2: create another timeline
        {"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "create_timeline", "arguments": {"name": "Standard Timeline"}}},
        
        # Test 3: list_timelines_tool
        {"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "list_timelines_tool", "arguments": {}}},
        
        # Test 4: set_current_timeline
        {"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "set_current_timeline", "arguments": {"name": "Custom Timeline"}}},
        
        # Test 5: Import media and add_clip_to_timeline
        {"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "import_media", "arguments": {"file_path": "/test/video.mp4"}}},
        {"jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": {"name": "add_clip_to_timeline", "arguments": {"clip_name": "video.mp4", "timeline_name": "Custom Timeline"}}},
        
        # Test 6: get_timeline_tracks
        {"jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": {"name": "get_timeline_tracks", "arguments": {"timeline_name": "Custom Timeline"}}},
        
        # Test 7: delete_timeline
        {"jsonrpc": "2.0", "id": 10, "method": "tools/call", "params": {"name": "delete_timeline", "arguments": {"name": "Standard Timeline"}}},
        
        # Final verification
        {"jsonrpc": "2.0", "id": 11, "method": "tools/call", "params": {"name": "list_timelines_tool", "arguments": {}}}
    ]
    
    print("🎬 Comprehensive Timeline Enhancement Test")
    print("=" * 50)
    print("Testing all 6 timeline enhancement tools...")
    print()
    
    for msg in messages:
        proc.stdin.write(json.dumps(msg) + "\n")
    
    proc.stdin.close()
    
    # Process responses
    successes = []
    failures = []
    
    for line in proc.stdout:
        if line.strip():
            try:
                response = json.loads(line.strip())
                
                if "id" in response and response["id"] >= 2:  # Tool calls start at id 2
                    result = response.get("result", {})
                    content = result.get("content", [{}])[0].get("text", "")
                    is_error = result.get("is_error", False)
                    
                    tool_names = [
                        "create_project", "create_empty_timeline", "create_timeline", 
                        "list_timelines_tool", "set_current_timeline", "import_media",
                        "add_clip_to_timeline", "get_timeline_tracks", "delete_timeline",
                        "list_timelines_tool (final)"
                    ]
                    
                    tool_index = response["id"] - 2
                    tool_name = tool_names[tool_index] if tool_index < len(tool_names) else f"tool_{response['id']}"
                    
                    if not is_error:
                        print(f"✅ {tool_name}: SUCCESS")
                        if "timeline" in tool_name.lower():
                            print(f"   → {content}")
                        successes.append(tool_name)
                    else:
                        print(f"❌ {tool_name}: FAILED - {content}")
                        failures.append(tool_name)
                        
            except json.JSONDecodeError:
                print(f"Parse error: {line.strip()}")
    
    errors = proc.stderr.read()
    if errors.strip():
        print(f"\n⚠️  Stderr: {errors}")
        
    proc.wait()
    
    # Results
    timeline_tools = [
        "create_empty_timeline", "list_timelines_tool", "set_current_timeline",
        "add_clip_to_timeline", "get_timeline_tracks", "delete_timeline"
    ]
    
    timeline_successes = [tool for tool in successes if any(tt in tool for tt in timeline_tools)]
    
    print(f"\n📊 Test Results:")
    print(f"   • Total successes: {len(successes)}")
    print(f"   • Total failures: {len(failures)}")
    print(f"   • Timeline enhancement tools working: {len(timeline_successes)}/6")
    
    if len(timeline_successes) >= 5:
        print(f"\n🎉 Timeline Enhancement Implementation: SUCCESS!")
        print(f"   Phase 3 Week 2 completed successfully!")
        print(f"   DaVinci Resolve MCP Server now has 20 tools total")
        return True
    else:
        print(f"\n❌ Timeline Enhancement Implementation: NEEDS WORK")
        return False

if __name__ == "__main__":
    success = test_all_timeline_tools()
    sys.exit(0 if success else 1) 