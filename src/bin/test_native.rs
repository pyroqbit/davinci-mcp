use davinci_mcp_rs::native::test_native_integration;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🧪 Testing Native DaVinci Resolve Integration");
    println!("============================================");
    
    // Test native integration
    match test_native_integration() {
        Ok(()) => {
            println!("✅ Native integration test completed successfully!");
        },
        Err(e) => {
            println!("❌ Native integration test failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
} 