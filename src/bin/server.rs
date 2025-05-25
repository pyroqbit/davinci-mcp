use davinci_mcp_rs::{DaVinciResolveServer, bridge::ConnectionMode};
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};
use tracing_subscriber;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Determine connection mode from environment variable
    let connection_mode = if env::var("DAVINCI_SIMULATION_MODE").unwrap_or_else(|_| "false".to_string()).to_lowercase() == "true" {
        ConnectionMode::Simulation
    } else {
        // Default to Real mode - try to connect to actual DaVinci Resolve
        ConnectionMode::Real
    };
    
    println!("Starting DaVinci Resolve MCP Server in {:?} mode", connection_mode);
    
    // Create the DaVinci Resolve MCP server with the determined mode
    let server = match connection_mode {
        ConnectionMode::Simulation => DaVinciResolveServer::new(),
        ConnectionMode::Real => DaVinciResolveServer::new_real(),
    };
    
    // Initialize the server
    if let Err(e) = server.initialize().await {
        eprintln!("Failed to initialize DaVinci Resolve connection: {}", e);
        eprintln!("Tip: Make sure DaVinci Resolve is running and 'External scripting using local network' is enabled in Preferences > System > General");
        return Err(e.into());
    }
    
    println!("DaVinci Resolve MCP Server initialized successfully");
    
    // Create stdio transport
    let transport = (stdin(), stdout());
    
    // Start the server
    server.serve(transport).await?;
    
    Ok(())
}
