#!/usr/bin/env python3
"""
Test script to verify DaVinci Resolve API connection
"""

import sys
import os

# Add DaVinci Resolve Python API path
resolve_api_path = "/opt/resolve/Developer/Scripting/Modules"
if resolve_api_path not in sys.path:
    sys.path.append(resolve_api_path)

try:
    import DaVinciResolveScript as dvr_script
    
    print("🔍 Testing DaVinci Resolve API connection...")
    
    # Connect to DaVinci Resolve
    resolve = dvr_script.scriptapp("Resolve")
    if resolve is None:
        print("❌ ERROR: DaVinci Resolve API not available")
        print("💡 Make sure 'External scripting using local network' is enabled in:")
        print("   DaVinci Resolve > Preferences > System > General")
        sys.exit(1)
    
    print("✅ SUCCESS: Connected to DaVinci Resolve")
    
    # Test project manager
    project_manager = resolve.GetProjectManager()
    if project_manager:
        print("✅ SUCCESS: Project manager accessible")
        
        # Get current project
        current_project = project_manager.GetCurrentProject()
        if current_project:
            project_name = current_project.GetName()
            print(f"✅ SUCCESS: Current project: '{project_name}'")
            
            # Test timeline access
            timeline_count = current_project.GetTimelineCount()
            print(f"✅ SUCCESS: Found {timeline_count} timelines")
            
            # Test media pool
            media_pool = current_project.GetMediaPool()
            if media_pool:
                print("✅ SUCCESS: Media pool accessible")
            
        else:
            print("⚠️  WARNING: No project currently open")
    
    print("\n🎉 DaVinci Resolve API is fully functional!")
    print("🚀 Your Rust MCP server should now work with real DaVinci Resolve operations")
    
except ImportError as e:
    print(f"❌ ERROR: Cannot import DaVinciResolveScript: {e}")
    print("💡 Make sure DaVinci Resolve is installed and the API module is available")
    sys.exit(1)
except Exception as e:
    print(f"❌ ERROR: {e}")
    sys.exit(1) 