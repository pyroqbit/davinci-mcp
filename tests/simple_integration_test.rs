#[cfg(test)]
mod tests {
    use davinci_mcp_rs::bridge::ResolveState;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_direct_davinci_resolve_operations() {
        println!("🎬 Direct DaVinci Resolve Integration Test");
        println!("{}", "=".repeat(50));

        // Initialize the bridge
        let state = Arc::new(Mutex::new(ResolveState::new()));

        println!("\n🔧 Test 1: Initialize connection");
        let result = state.lock().unwrap().initialize().await;
        assert!(result.is_ok(), "Failed to initialize: {:?}", result);
        println!("✅ Successfully connected to DaVinci Resolve");

        println!("\n📄 Test 2: Switch to Edit page");
        let result = state.lock().unwrap().switch_page("edit").await;
        assert!(result.is_ok(), "Failed to switch page: {:?}", result);
        println!("✅ Switched to Edit page: {}", result.unwrap());

        println!("\n📁 Test 3: Create timeline");
        let timeline_args = serde_json::json!({
            "name": "Rust Integration Test Timeline",
            "frame_rate": "24",
            "resolution_width": 1920,
            "resolution_height": 1080
        });

        let result = state
            .lock()
            .unwrap()
            .create_empty_timeline(timeline_args)
            .await;
        assert!(result.is_ok(), "Failed to create timeline: {:?}", result);
        println!("✅ Created timeline: {}", result.unwrap());

        println!("\n🎯 Test 4: Add marker");
        let marker_args = serde_json::json!({
            "frame": 50,
            "color": "Blue",
            "note": "Rust Integration Test Marker"
        });

        let result = state.lock().unwrap().add_marker(marker_args).await;
        assert!(result.is_ok(), "Failed to add marker: {:?}", result);
        println!("✅ Added marker: {}", result.unwrap());

        println!("\n📋 Test 5: List timelines");
        let result = state.lock().unwrap().list_timelines().await;
        assert!(result.is_ok(), "Failed to list timelines: {:?}", result);
        println!("✅ Timelines: {}", result.unwrap());

        println!("\n✅ All integration tests completed!");
    }
}
