use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::{json, Value};

use rmcp::{
    model::{
        CallToolResult, ClientNotification, ClientRequest, Content, Implementation,
        InitializeResult, ListToolsResult, ProtocolVersion, ServerCapabilities,
        Tool, CallToolRequestParam, ServerResult
    },
    service::{RequestContext, Service},
    Error as McpError, RoleServer,
};

use crate::{
    bridge::ResolveBridge,
    config::Config,
    error::ResolveError,
    tools::{handle_tool_call},
};

/// Main DaVinci Resolve MCP Server
#[derive(Debug)]
pub struct DaVinciResolveServer {
    /// Configuration
    config: Arc<Config>,
    /// Python bridge to DaVinci Resolve
    bridge: Arc<ResolveBridge>,
    /// Server initialized flag
    initialized: Arc<RwLock<bool>>,
}

impl DaVinciResolveServer {
    /// Create a new server instance with default configuration
    pub fn new() -> Self {
        let config = Config::default();
        Self::with_config(config)
    }

    /// Create a new server instance with custom configuration
    pub fn with_config(config: Config) -> Self {
        let bridge = Arc::new(ResolveBridge::new());
        Self {
            config: Arc::new(config),
            bridge,
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the server and DaVinci Resolve connection
    pub async fn initialize(&self) -> Result<(), ResolveError> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        // Initialize Python bridge
        self.bridge.initialize().await?;

        *initialized = true;
        Ok(())
    }

    /// Handle MCP tool calls by routing to the centralized handler
    pub async fn handle_tool_call(&self, name: &str, arguments: Option<serde_json::Map<String, Value>>) -> Result<String, ResolveError> {
        // Convert arguments to Value for the handler
        let args = match arguments {
            Some(args_map) => Value::Object(args_map),
            None => json!({}),
        };

        // Use the centralized tool handler
        handle_tool_call(name, args, self.bridge.clone()).await
    }

    /// Get list of all available tools with comprehensive schemas
    fn get_tools(&self) -> Vec<Tool> {
        vec![
            // ==================== PHASE 1 & 2 TOOLS ====================
            
            // Project Management
            Tool::new(
                "create_project",
                "Create a new DaVinci Resolve project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name for the new project"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "open_project",
                "Open an existing DaVinci Resolve project by name",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the project to open"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "switch_page",
                "Switch to a specific page in DaVinci Resolve",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "page": {
                            "type": "string",
                            "description": "The page to switch to",
                            "enum": ["media", "cut", "edit", "fusion", "color", "fairlight", "deliver"]
                        }
                    },
                    "required": ["page"]
                }).as_object().unwrap().clone()),
            ),

            // Timeline Operations
            Tool::new(
                "create_timeline",
                "Create a new timeline with optional custom settings",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name for the new timeline"
                        },
                        "frame_rate": {
                            "type": "string",
                            "description": "Optional frame rate (e.g. '24', '29.97', '30', '60')"
                        },
                        "resolution_width": {
                            "type": "integer",
                            "description": "Optional width in pixels (e.g. 1920)"
                        },
                        "resolution_height": {
                            "type": "integer",
                            "description": "Optional height in pixels (e.g. 1080)"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "add_marker",
                "Add a colored marker to the timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "frame": {
                            "type": "integer",
                            "description": "Frame number to add the marker at (defaults to current position if not specified)"
                        },
                        "color": {
                            "type": "string",
                            "description": "Marker color",
                            "enum": ["Blue", "Cyan", "Green", "Yellow", "Red", "Pink", "Purple", "Fuchsia", "Rose", "Lavender", "Sky", "Mint", "Lemon", "Sand", "Cocoa", "Cream"],
                            "default": "Blue"
                        },
                        "note": {
                            "type": "string",
                            "description": "Text note to add to the marker",
                            "default": ""
                        }
                    },
                    "required": ["color", "note"]
                }).as_object().unwrap().clone()),
            ),

            // Basic Media Operations  
            Tool::new(
                "import_media",
                "Import media file into the current project's media pool",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the media file to import"
                        }
                    },
                    "required": ["file_path"]
                }).as_object().unwrap().clone()),
            ),

            // ==================== PHASE 3 WEEK 1: MEDIA OPERATIONS ====================

            Tool::new(
                "create_bin",
                "Create a new bin/folder in the media pool",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name for the new bin"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "auto_sync_audio",
                "Sync audio between clips with customizable settings",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_names": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "List of clip names to sync"
                        },
                        "sync_method": {
                            "type": "string",
                            "description": "Method to use for synchronization - 'waveform' or 'timecode'",
                            "enum": ["waveform", "timecode"],
                            "default": "waveform"
                        },
                        "append_mode": {
                            "type": "boolean",
                            "description": "Whether to append the audio or replace it",
                            "default": false
                        },
                        "target_bin": {
                            "type": "string",
                            "description": "Optional bin to move synchronized clips to"
                        }
                    },
                    "required": ["clip_names"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "unlink_clips",
                "Unlink specified clips, disconnecting them from their media files",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_names": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "List of clip names to unlink"
                        }
                    },
                    "required": ["clip_names"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "relink_clips",
                "Relink specified clips to their media files",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_names": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "List of clip names to relink"
                        },
                        "media_paths": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Optional list of specific media file paths to use for relinking"
                        },
                        "folder_path": {
                            "type": "string",
                            "description": "Optional folder path to search for media files"
                        },
                        "recursive": {
                            "type": "boolean",
                            "description": "Whether to search the folder path recursively",
                            "default": false
                        }
                    },
                    "required": ["clip_names"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "create_sub_clip",
                "Create a subclip from the specified clip using in and out points",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the source clip"
                        },
                        "start_frame": {
                            "type": "integer",
                            "description": "Start frame (in point)"
                        },
                        "end_frame": {
                            "type": "integer",
                            "description": "End frame (out point)"
                        },
                        "sub_clip_name": {
                            "type": "string",
                            "description": "Optional name for the subclip (defaults to original name with '_subclip')"
                        },
                        "bin_name": {
                            "type": "string",
                            "description": "Optional bin to place the subclip in"
                        }
                    },
                    "required": ["clip_name", "start_frame", "end_frame"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "link_proxy_media",
                "Link a proxy media file to a clip",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to link proxy to"
                        },
                        "proxy_file_path": {
                            "type": "string",
                            "description": "Path to the proxy media file"
                        }
                    },
                    "required": ["clip_name", "proxy_file_path"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "unlink_proxy_media",
                "Unlink proxy media from a clip",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to unlink proxy from"
                        }
                    },
                    "required": ["clip_name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "replace_clip",
                "Replace a clip with another media file",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to be replaced"
                        },
                        "replacement_path": {
                            "type": "string",
                            "description": "Path to the replacement media file"
                        }
                    },
                    "required": ["clip_name", "replacement_path"]
                }).as_object().unwrap().clone()),
            ),
        ]
    }
}

impl Service<RoleServer> for DaVinciResolveServer {
    async fn handle_request(
        &self,
        request: ClientRequest,
        _context: RequestContext<RoleServer>,
    ) -> Result<ServerResult, McpError> {
        match request {
            ClientRequest::InitializeRequest(_) => {
                // Handle initialization
                let info = self.get_info();
                Ok(ServerResult::InitializeResult(info))
            }
            ClientRequest::ListToolsRequest(_) => {
                let tools = self.get_tools();
                Ok(ServerResult::ListToolsResult(ListToolsResult { 
                    tools,
                    next_cursor: None
                }))
            }
            ClientRequest::CallToolRequest(call_tool_request) => {
                // Extract the actual parameters from the request
                let CallToolRequestParam { name, arguments } = call_tool_request.params;
                
                match self.handle_tool_call(&name, arguments).await {
                    Ok(content) => Ok(ServerResult::CallToolResult(CallToolResult {
                        content: vec![Content::text(content)],
                        is_error: Some(false),
                    })),
                    Err(e) => Ok(ServerResult::CallToolResult(CallToolResult {
                        content: vec![Content::text(format!("Error: {}", e))],
                        is_error: Some(true),
                    })),
                }
            }
            _ => {
                // Create a proper method not found error
                Err(McpError::method_not_found::<rmcp::model::JsonRpcVersion2_0>())
            }
        }
    }

    async fn handle_notification(
        &self,
        _notification: ClientNotification,
    ) -> Result<(), McpError> {
        // Handle notifications if needed
        Ok(())
    }

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities {
                tools: Some(Default::default()),
                ..Default::default()
            },
            server_info: Implementation {
                name: "davinci-resolve-mcp".into(),
                version: "2.0.0".into(),
            },
            instructions: Some("DaVinci Resolve MCP Server (Rust) - Automate DaVinci Resolve workflows with 14 tools including project management, timeline operations, and advanced media pool management".to_string()),
        }
    }
}

impl Default for DaVinciResolveServer {
    fn default() -> Self {
        Self::new()
    }
} 