#[cfg(test)]
mod tests {
    use std::process::Command;
    use serde_json::Value;

    #[tokio::test]
    async fn test_real_davinci_resolve_api() {
        println!("🎬 ЧЕСТНЫЙ тест реального DaVinci Resolve API");
        println!("{}", "=".repeat(60));

        // Тест 1: Проверяем что DaVinci Resolve запущен
        println!("\n🔍 Test 1: Checking if DaVinci Resolve is running");
        let ps_output = Command::new("ps")
            .args(&["aux"])
            .output()
            .expect("Failed to run ps command");
        
        let ps_string = String::from_utf8_lossy(&ps_output.stdout);
        if !ps_string.contains("/opt/resolve/bin/resolve") {
            panic!("❌ DaVinci Resolve is not running! Start it first.");
        }
        println!("✅ DaVinci Resolve is running");

        // Тест 2: Попытка подключения через Python API
        println!("\n🐍 Test 2: Attempting real Python API connection");
        let python_script = r#"
import sys
import os

# Add DaVinci Resolve Python API path
resolve_api_path = "/opt/resolve/Developer/Scripting/Modules"
if resolve_api_path not in sys.path:
    sys.path.append(resolve_api_path)

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if resolve is None:
        print("ERROR: Cannot connect to DaVinci Resolve")
        print("HINT: Enable 'External scripting using local network' in DaVinci Resolve preferences")
        sys.exit(1)
    
    print("SUCCESS: Connected to DaVinci Resolve")
    
    # Get project manager
    project_manager = resolve.GetProjectManager()
    if not project_manager:
        print("ERROR: Cannot get project manager")
        sys.exit(1)
    
    print("SUCCESS: Got project manager")
    
    # Get current project
    project = project_manager.GetCurrentProject()
    if not project:
        print("ERROR: No project is open")
        print("HINT: Open a project in DaVinci Resolve")
        sys.exit(1)
    
    project_name = project.GetName()
    print(f"SUCCESS: Current project: {project_name}")
    
    # Try to switch to Edit page
    print("ATTEMPTING: Switch to Edit page")
    result = resolve.OpenPage("edit")
    print(f"RESULT: OpenPage returned {result}")
    
    # Try to get current timeline
    timeline = project.GetCurrentTimeline()
    if timeline:
        timeline_name = timeline.GetName()
        print(f"SUCCESS: Current timeline: {timeline_name}")
        
        # Try to add a marker
        print("ATTEMPTING: Add marker to timeline")
        marker_result = timeline.AddMarker(100, "Red", "Rust Test Marker", "Added by Rust test", 1)
        print(f"RESULT: AddMarker returned {marker_result}")
        
        if marker_result:
            print("🎉 SUCCESS: Marker added to timeline!")
        else:
            print("❌ FAILED: Could not add marker")
    else:
        print("WARNING: No timeline is selected")
    
    print("SUCCESS: All API calls completed")
    
except ImportError as e:
    print(f"ERROR: Cannot import DaVinciResolveScript: {e}")
    sys.exit(1)
except Exception as e:
    print(f"ERROR: {e}")
    sys.exit(1)
"#;

        // Execute Python script
        let output = Command::new("python3")
            .arg("-c")
            .arg(python_script)
            .output()
            .expect("Failed to execute Python script");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        println!("📤 Python output:");
        println!("{}", stdout);
        
        if !stderr.is_empty() {
            println!("⚠️ Python errors:");
            println!("{}", stderr);
        }

        // Check if the script succeeded
        if !output.status.success() {
            println!("❌ FAILED: Python script failed with exit code: {}", output.status);
            println!("💡 This means we cannot actually connect to DaVinci Resolve");
            panic!("Real API test failed - cannot connect to DaVinci Resolve");
        }

        if stdout.contains("SUCCESS: All API calls completed") {
            println!("🎉 SUCCESS: Real DaVinci Resolve API is working!");
            println!("🔥 Check DaVinci Resolve - you should see a red marker at frame 100!");
        } else {
            println!("❌ FAILED: API calls did not complete successfully");
            panic!("Real API test failed - API calls incomplete");
        }
    }

    #[tokio::test]
    async fn test_rust_to_python_bridge() {
        println!("🦀 Testing Rust to Python bridge for DaVinci Resolve");
        println!("{}", "=".repeat(60));

        // This test shows how we could call DaVinci Resolve from Rust
        // by executing Python scripts and parsing results
        
        let python_script = r#"
import sys
import os
import json

resolve_api_path = "/opt/resolve/Developer/Scripting/Modules"
if resolve_api_path not in sys.path:
    sys.path.append(resolve_api_path)

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    
    if not resolve:
        print(json.dumps({"error": "Cannot connect to DaVinci Resolve"}))
        sys.exit(1)
    
    project_manager = resolve.GetProjectManager()
    project = project_manager.GetCurrentProject()
    
    if not project:
        print(json.dumps({"error": "No project open"}))
        sys.exit(1)
    
    # Get project info
    project_name = project.GetName()
    timeline_count = project.GetTimelineCount()
    
    # Get timeline info
    timelines = []
    for i in range(1, timeline_count + 1):
        timeline = project.GetTimelineByIndex(i)
        if timeline:
            timelines.append({
                "name": timeline.GetName(),
                "frame_rate": timeline.GetSetting("timelineFrameRate"),
                "resolution": f"{timeline.GetSetting('timelineResolutionWidth')}x{timeline.GetSetting('timelineResolutionHeight')}"
            })
    
    result = {
        "success": True,
        "project_name": project_name,
        "timeline_count": timeline_count,
        "timelines": timelines
    }
    
    print(json.dumps(result))
    
except Exception as e:
    print(json.dumps({"error": str(e)}))
    sys.exit(1)
"#;

        let output = Command::new("python3")
            .arg("-c")
            .arg(python_script)
            .output()
            .expect("Failed to execute Python script");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            match serde_json::from_str::<Value>(&stdout) {
                Ok(json_result) => {
                    if json_result.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                        println!("✅ Successfully got project info from DaVinci Resolve:");
                        println!("📁 Project: {}", json_result["project_name"].as_str().unwrap_or("Unknown"));
                        println!("📊 Timeline count: {}", json_result["timeline_count"].as_i64().unwrap_or(0));
                        
                        if let Some(timelines) = json_result["timelines"].as_array() {
                            for timeline in timelines {
                                println!("🎬 Timeline: {} ({})", 
                                    timeline["name"].as_str().unwrap_or("Unknown"),
                                    timeline["resolution"].as_str().unwrap_or("Unknown")
                                );
                            }
                        }
                        
                        println!("🎉 Rust successfully communicated with DaVinci Resolve via Python!");
                    } else {
                        let error = json_result["error"].as_str().unwrap_or("Unknown error");
                        println!("❌ DaVinci Resolve API error: {}", error);
                        panic!("DaVinci Resolve API error: {}", error);
                    }
                }
                Err(e) => {
                    println!("❌ Failed to parse JSON response: {}", e);
                    println!("Raw output: {}", stdout);
                    panic!("Failed to parse DaVinci Resolve response");
                }
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("❌ Python script failed: {}", stderr);
            panic!("Python script execution failed");
        }
    }
} 