use davinci_mcp_rs::bridge::ResolveState;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Direct DaVinci Resolve Test (Pure Rust)");
    println!("{}", "=".repeat(50));

    // Initialize the bridge
    let state = Arc::new(Mutex::new(ResolveState::new()));
    
    println!("\n🔧 Test 1: Initialize connection");
    match state.lock().unwrap().initialize().await {
        Ok(_) => println!("✅ Successfully connected to DaVinci Resolve"),
        Err(e) => {
            println!("❌ Failed to connect: {}", e);
            return Ok(());
        }
    }

    println!("\n📄 Test 2: Switch to Edit page");
    match state.lock().unwrap().switch_page("edit").await {
        Ok(result) => println!("✅ Switched to Edit page: {}", result),
        Err(e) => println!("❌ Failed to switch page: {}", e),
    }

    println!("\n📁 Test 3: Create timeline");
    let timeline_args = serde_json::json!({
        "name": "Rust Direct Test Timeline",
        "frame_rate": "24",
        "resolution_width": 1920,
        "resolution_height": 1080
    });
    
    match state.lock().unwrap().create_empty_timeline(timeline_args).await {
        Ok(result) => println!("✅ Created timeline: {}", result),
        Err(e) => println!("❌ Failed to create timeline: {}", e),
    }

    println!("\n🎯 Test 4: Add marker");
    let marker_args = serde_json::json!({
        "frame": 50,
        "color": "Blue",
        "note": "Rust Direct Test Marker"
    });
    
    match state.lock().unwrap().add_marker(marker_args).await {
        Ok(result) => println!("✅ Added marker: {}", result),
        Err(e) => println!("❌ Failed to add marker: {}", e),
    }

    println!("\n📋 Test 5: List timelines");
    match state.lock().unwrap().list_timelines().await {
        Ok(result) => println!("✅ Timelines: {}", result),
        Err(e) => println!("❌ Failed to list timelines: {}", e),
    }

    println!("\n✅ All direct tests completed!");
    Ok(())
} 