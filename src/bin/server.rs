use davinci_mcp_rs::DaVinciResolveServer;
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Create the DaVinci Resolve MCP server
    let server = DaVinciResolveServer::new();
    
    // Create stdio transport
    let transport = (stdin(), stdout());
    
    // Start the server
    server.serve(transport).await?;
    
    Ok(())
}
