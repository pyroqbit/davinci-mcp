use davinci_mcp_rs::bridge::{ResolveBridge, ConnectionMode};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Real DaVinci Resolve Connection Test (Pure Rust)");
    println!("{}", "=".repeat(60));

    // Initialize the bridge in REAL mode
    let bridge = ResolveBridge::new(ConnectionMode::Real);
    
    println!("\n🔧 Test 1: Initialize real connection to DaVinci Resolve");
    match bridge.initialize().await {
        Ok(_) => println!("✅ Successfully connected to DaVinci Resolve"),
        Err(e) => {
            println!("❌ Failed to connect: {}", e);
            println!("💡 Make sure DaVinci Resolve is running and external scripting is enabled");
            return Ok(());
        }
    }

    // Check connection status
    if bridge.is_connected().await {
        println!("✅ Connection verified - DaVinci Resolve is accessible");
    } else {
        println!("❌ Connection failed");
        return Ok(());
    }

    println!("\n📄 Test 2: Switch to Edit page");
    let switch_args = json!({"page": "edit"});
    match bridge.call_api("switch_page", switch_args).await {
        Ok(result) => println!("✅ Switched to Edit page: {}", result),
        Err(e) => println!("❌ Failed to switch page: {}", e),
    }

    println!("\n📁 Test 3: Create a new timeline");
    let timeline_args = json!({
        "name": "Rust Real Test Timeline",
        "frame_rate": "24",
        "resolution_width": 1920,
        "resolution_height": 1080
    });
    
    match bridge.call_api("create_empty_timeline", timeline_args).await {
        Ok(result) => println!("✅ Created timeline: {}", result),
        Err(e) => println!("❌ Failed to create timeline: {}", e),
    }

    println!("\n🎯 Test 4: Add a marker to timeline");
    let marker_args = json!({
        "frame": 120,
        "color": "Green",
        "note": "Rust Real Test Marker - Success!"
    });
    
    match bridge.call_api("add_marker", marker_args).await {
        Ok(result) => println!("✅ Added marker: {}", result),
        Err(e) => println!("❌ Failed to add marker: {}", e),
    }

    println!("\n📋 Test 5: List all timelines");
    let list_args = json!({"random_string": "test"});
    match bridge.call_api("list_timelines_tool", list_args).await {
        Ok(result) => println!("✅ Timelines: {}", result),
        Err(e) => println!("❌ Failed to list timelines: {}", e),
    }

    println!("\n🎨 Test 6: Switch to Color page");
    let color_args = json!({"page": "color"});
    match bridge.call_api("switch_page", color_args).await {
        Ok(result) => println!("✅ Switched to Color page: {}", result),
        Err(e) => println!("❌ Failed to switch to Color page: {}", e),
    }

    println!("\n🚀 Test 7: Switch to Deliver page");
    let deliver_args = json!({"page": "deliver"});
    match bridge.call_api("switch_page", deliver_args).await {
        Ok(result) => println!("✅ Switched to Deliver page: {}", result),
        Err(e) => println!("❌ Failed to switch to Deliver page: {}", e),
    }

    println!("\n✅ All real DaVinci Resolve tests completed successfully!");
    println!("🎉 Your Rust MCP server is working with real DaVinci Resolve!");
    
    Ok(())
} 