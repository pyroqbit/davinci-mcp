#[cfg(test)]
mod tests {
    use davinci_mcp_rs::bridge::{ConnectionMode, ResolveBridge};
    use serde_json::json;

    #[tokio::test]
    async fn test_real_davinci_resolve_connection() {
        println!("🎬 Real DaVinci Resolve Connection Integration Test");
        println!("{}", "=".repeat(60));

        // Initialize the bridge in REAL mode
        let bridge = ResolveBridge::new(ConnectionMode::Real);

        println!("\n🔧 Test 1: Initialize real connection to DaVinci Resolve");
        let init_result = bridge.initialize().await;

        // ЧЕСТНАЯ ПРОВЕРКА - если DaVinci Resolve не запущен, тест должен ПРОВАЛИТЬСЯ
        match init_result {
            Ok(()) => {
                println!("✅ Successfully connected to DaVinci Resolve");
                assert!(
                    bridge.is_connected().await,
                    "Connection should be established"
                );
                println!("✅ Connection verified - DaVinci Resolve is accessible");
            }
            Err(e) => {
                println!("❌ FAILED: Cannot connect to DaVinci Resolve: {}", e);
                println!(
                    "💡 Make sure DaVinci Resolve is running and external scripting is enabled"
                );
                println!("💡 This test requires a REAL DaVinci Resolve instance");
                panic!(
                    "Real connection test failed - DaVinci Resolve not available: {}",
                    e
                );
            }
        }

        println!("\n📄 Test 2: Switch to Edit page");
        let switch_args = json!({"page": "edit"});
        let result = bridge.call_api("switch_page", switch_args).await;
        match result {
            Ok(response) => {
                println!("✅ Switched to Edit page: {}", response);
            }
            Err(e) => {
                println!("❌ FAILED to switch page: {}", e);
                panic!("Real API call failed: {}", e);
            }
        }

        println!("\n📁 Test 3: Create a new timeline");
        let timeline_args = json!({
            "name": "Rust Real Integration Test Timeline",
            "frame_rate": "24",
            "resolution_width": 1920,
            "resolution_height": 1080
        });

        let result = bridge
            .call_api("create_empty_timeline", timeline_args)
            .await;
        match result {
            Ok(response) => {
                println!("✅ Created timeline: {}", response);
            }
            Err(e) => {
                println!("❌ FAILED to create timeline: {}", e);
                panic!("Real API call failed: {}", e);
            }
        }

        println!("\n🎯 Test 4: Add a marker to timeline");
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
                println!("❌ FAILED to add marker: {}", e);
                panic!("Real API call failed: {}", e);
            }
        }

        println!("\n📋 Test 5: List all timelines");
        let list_args = json!({"random_string": "test"});
        let result = bridge.call_api("list_timelines_tool", list_args).await;
        match result {
            Ok(response) => {
                println!("✅ Timelines: {}", response);
            }
            Err(e) => {
                println!("❌ FAILED to list timelines: {}", e);
                panic!("Real API call failed: {}", e);
            }
        }

        println!("\n🎨 Test 6: Switch to Color page");
        let color_args = json!({"page": "color"});
        let result = bridge.call_api("switch_page", color_args).await;
        match result {
            Ok(response) => {
                println!("✅ Switched to Color page: {}", response);
            }
            Err(e) => {
                println!("❌ FAILED to switch to Color page: {}", e);
                panic!("Real API call failed: {}", e);
            }
        }

        println!("\n🚀 Test 7: Switch to Deliver page");
        let deliver_args = json!({"page": "deliver"});
        let result = bridge.call_api("switch_page", deliver_args).await;
        match result {
            Ok(response) => {
                println!("✅ Switched to Deliver page: {}", response);
            }
            Err(e) => {
                println!("❌ FAILED to switch to Deliver page: {}", e);
                panic!("Real API call failed: {}", e);
            }
        }

        println!("\n✅ Real DaVinci Resolve integration test completed!");
        println!("🎉 Your Rust MCP server successfully connected to DaVinci Resolve!");
        println!("🔥 ALL TESTS PASSED - Native Rust integration working!");
    }
}
