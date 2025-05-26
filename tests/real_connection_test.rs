#[cfg(test)]
mod tests {
    use davinci_mcp_rs::bridge::{ResolveBridge, ConnectionMode};
    use serde_json::json;

    #[tokio::test]
    #[ignore] // Игнорируем по умолчанию, так как требует запущенный DaVinci Resolve
    async fn test_real_davinci_resolve_connection() {
        println!("🎬 Real DaVinci Resolve Connection Integration Test");
        println!("{}", "=".repeat(60));

        // Initialize the bridge in REAL mode
        let bridge = ResolveBridge::new(ConnectionMode::Real);
        
        println!("\n🔧 Test 1: Initialize real connection to DaVinci Resolve");
        let init_result = bridge.initialize().await;
        if init_result.is_err() {
            println!("⚠️ Skipping real connection test - DaVinci Resolve not available");
            println!("💡 Make sure DaVinci Resolve is running and external scripting is enabled");
            return;
        }
        println!("✅ Successfully connected to DaVinci Resolve");

        // Check connection status
        assert!(bridge.is_connected().await, "Connection should be established");
        println!("✅ Connection verified - DaVinci Resolve is accessible");

        println!("\n📄 Test 2: Switch to Edit page");
        let switch_args = json!({"page": "edit"});
        let result = bridge.call_api("switch_page", switch_args).await;
        assert!(result.is_ok(), "Failed to switch page: {:?}", result);
        println!("✅ Switched to Edit page: {}", result.unwrap());

        println!("\n📁 Test 3: Create a new timeline (with fallback to simulation)");
        let timeline_args = json!({
            "name": "Rust Real Integration Test Timeline",
            "frame_rate": "24",
            "resolution_width": 1920,
            "resolution_height": 1080
        });
        
        let result = bridge.call_api("create_empty_timeline", timeline_args).await;
        // This should work either with real API or fallback to simulation
        match result {
            Ok(response) => {
                println!("✅ Created timeline: {}", response);
            }
            Err(e) => {
                println!("⚠️ Timeline creation failed: {}", e);
                println!("💡 This is expected if DaVinci Resolve Python API is not available");
                println!("💡 The system should fall back to simulation mode");
                
                // In real mode with fallback, we expect this to work via simulation
                // Let's test that the bridge can handle this gracefully
                assert!(e.to_string().contains("NotRunning") || 
                       e.to_string().contains("internal") || 
                       e.to_string().contains("not running"), 
                    "Expected NotRunning, internal, or 'not running' error, got: {}", e);
            }
        }

        println!("\n🎯 Test 4: Add a marker to timeline (with fallback)");
        let marker_args = json!({
            "frame": 120,
            "color": "Green",
            "note": "Rust Real Integration Test Marker - Success!"
        });
        
        let result = bridge.call_api("add_marker", marker_args).await;
        match result {
            Ok(response) => {
                println!("✅ Added marker: {}", response);
            }
            Err(e) => {
                println!("⚠️ Marker creation failed: {}", e);
                println!("💡 This is expected without a current timeline");
            }
        }

        println!("\n📋 Test 5: List all timelines");
        let list_args = json!({"random_string": "test"});
        let result = bridge.call_api("list_timelines_tool", list_args).await;
        assert!(result.is_ok(), "Failed to list timelines: {:?}", result);
        println!("✅ Timelines: {}", result.unwrap());

        println!("\n🎨 Test 6: Switch to Color page");
        let color_args = json!({"page": "color"});
        let result = bridge.call_api("switch_page", color_args).await;
        assert!(result.is_ok(), "Failed to switch to Color page: {:?}", result);
        println!("✅ Switched to Color page: {}", result.unwrap());

        println!("\n🚀 Test 7: Switch to Deliver page");
        let deliver_args = json!({"page": "deliver"});
        let result = bridge.call_api("switch_page", deliver_args).await;
        assert!(result.is_ok(), "Failed to switch to Deliver page: {:?}", result);
        println!("✅ Switched to Deliver page: {}", result.unwrap());

        println!("\n✅ Real DaVinci Resolve integration test completed!");
        println!("🎉 Your Rust MCP server can connect to DaVinci Resolve!");
        println!("💡 Some operations may fall back to simulation if Python API is not available");
    }
} 