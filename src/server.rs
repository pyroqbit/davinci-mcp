use std::sync::Arc;
use rmcp::{ServerCapabilities, Tool, Error as McpError, McpService};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument};

use crate::bridge::ResolveBridge;
use crate::config::Config;
use crate::error::{ResolveError, ResolveResult};
use crate::tools::*;

/// Main DaVinci Resolve MCP Server
pub struct DaVinciResolveServer {
    /// Configuration
    config: Arc<Config>,
    /// Python bridge to DaVinci Resolve
    bridge: Arc<ResolveBridge>,
    /// Project management tools
    project_tools: Arc<ProjectTools>,
    /// Timeline management tools
    timeline_tools: Arc<TimelineTools>,
    /// Media pool tools
    media_tools: Arc<MediaTools>,
    /// Color grading tools
    color_tools: Arc<ColorTools>,
    /// Rendering tools
    render_tools: Arc<RenderTools>,
    /// Export tools
    export_tools: Arc<ExportTools>,
    /// Server initialized flag
    initialized: Arc<RwLock<bool>>,
}

impl DaVinciResolveServer {
    /// Create a new DaVinci Resolve MCP server
    pub fn new(config: Config) -> Self {
        let bridge = Arc::new(ResolveBridge::new());
        
        Self {
            config: Arc::new(config),
            project_tools: Arc::new(ProjectTools::new(bridge.clone())),
            timeline_tools: Arc::new(TimelineTools::new(bridge.clone())),
            media_tools: Arc::new(MediaTools::new(bridge.clone())),
            color_tools: Arc::new(ColorTools::new(bridge.clone())),
            render_tools: Arc::new(RenderTools::new(bridge.clone())),
            export_tools: Arc::new(ExportTools::new(bridge.clone())),
            bridge,
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the server and Python bridge
    #[instrument(skip(self))]
    pub async fn initialize(&self) -> ResolveResult<()> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        info!("Initializing DaVinci Resolve MCP Server");
        
        // Initialize Python bridge
        self.bridge.initialize().await?;
        
        // Check if DaVinci Resolve is running
        if !self.bridge.is_resolve_running().await? {
            return Err(ResolveError::NotRunning);
        }
        
        *initialized = true;
        info!("DaVinci Resolve MCP Server initialized successfully");
        Ok(())
    }

    /// Get server capabilities
    pub fn get_capabilities(&self) -> ServerCapabilities {
        ServerCapabilities {
            tools: Some(mcp::ToolsCapability {}),
            ..Default::default()
        }
    }

    /// Get all available tools
    #[instrument(skip(self))]
    pub async fn get_tools(&self) -> Vec<Tool> {
        vec![
            // Project management tools
            Tool {
                name: "create_project".to_string(),
                description: Some("Create a new project with the given name".to_string()),
                input_schema: self.get_schema_for_type::<CreateProjectRequest>(),
            },
            Tool {
                name: "open_project".to_string(),
                description: Some("Open an existing project by name".to_string()),
                input_schema: self.get_schema_for_type::<OpenProjectRequest>(),
            },
            Tool {
                name: "save_project".to_string(),
                description: Some("Save the current project".to_string()),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            Tool {
                name: "close_project".to_string(),
                description: Some("Close the current project".to_string()),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            Tool {
                name: "set_project_setting".to_string(),
                description: Some("Set a project setting to the specified value".to_string()),
                input_schema: self.get_schema_for_type::<SetProjectSettingRequest>(),
            },
            Tool {
                name: "switch_page".to_string(),
                description: Some("Switch to a specific page in DaVinci Resolve".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "page": {
                            "type": "string",
                            "enum": ["media", "cut", "edit", "fusion", "color", "fairlight", "deliver"],
                            "description": "The page to switch to"
                        }
                    },
                    "required": ["page"]
                }),
            },
            
            // Timeline tools
            Tool {
                name: "create_timeline".to_string(),
                description: Some("Create a new timeline with the given name".to_string()),
                input_schema: self.get_schema_for_type::<CreateTimelineRequest>(),
            },
            Tool {
                name: "create_empty_timeline".to_string(),
                description: Some("Create a new timeline with custom settings".to_string()),
                input_schema: self.get_schema_for_type::<CreateTimelineRequest>(),
            },
            Tool {
                name: "delete_timeline".to_string(),
                description: Some("Delete a timeline by name".to_string()),
                input_schema: self.get_schema_for_type::<DeleteTimelineRequest>(),
            },
            Tool {
                name: "set_current_timeline".to_string(),
                description: Some("Switch to a timeline by name".to_string()),
                input_schema: self.get_schema_for_type::<SetCurrentTimelineRequest>(),
            },
            Tool {
                name: "add_marker".to_string(),
                description: Some("Add a marker at the specified frame in the current timeline".to_string()),
                input_schema: self.get_schema_for_type::<AddMarkerRequest>(),
            },
            Tool {
                name: "list_timelines".to_string(),
                description: Some("List all timelines in the current project".to_string()),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            
            // Media pool tools
            Tool {
                name: "import_media".to_string(),
                description: Some("Import media file into the current project's media pool".to_string()),
                input_schema: self.get_schema_for_type::<ImportMediaRequest>(),
            },
            Tool {
                name: "create_bin".to_string(),
                description: Some("Create a new bin/folder in the media pool".to_string()),
                input_schema: self.get_schema_for_type::<CreateBinRequest>(),
            },
        ]
    }

    /// Execute a tool call
    #[instrument(skip(self, arguments))]
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value, McpError> {
        // Ensure server is initialized
        if let Err(e) = self.initialize().await {
            return Err(McpError::InternalError(format!("Server initialization failed: {}", e)));
        }

        debug!("Executing tool: {} with arguments: {}", name, arguments);

        let result = match name {
            // Project management
            "create_project" => {
                let req: CreateProjectRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.project_tools.create_project(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "open_project" => {
                let req: OpenProjectRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.project_tools.open_project(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "save_project" => {
                self.project_tools.save_project().await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "close_project" => {
                self.project_tools.close_project().await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "set_project_setting" => {
                let req: SetProjectSettingRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.project_tools.set_project_setting(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "switch_page" => {
                let page = arguments.get("page")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| McpError::InvalidParams("Missing or invalid 'page' parameter".to_string()))?;
                self.project_tools.switch_page(page).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            
            // Timeline management
            "create_timeline" => {
                let req: CreateTimelineRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.timeline_tools.create_timeline(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "create_empty_timeline" => {
                let req: CreateTimelineRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.timeline_tools.create_empty_timeline(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "delete_timeline" => {
                let req: DeleteTimelineRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.timeline_tools.delete_timeline(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "set_current_timeline" => {
                let req: SetCurrentTimelineRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.timeline_tools.set_current_timeline(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "add_marker" => {
                let req: AddMarkerRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.timeline_tools.add_marker(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "list_timelines" => {
                self.timeline_tools.list_timelines().await
                    .map(|timelines| json!({"content": [{"type": "text", "text": format!("Timelines: {}", timelines.join(", "))}]}))
            }
            
            // Media pool management
            "import_media" => {
                let req: ImportMediaRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.media_tools.import_media(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            "create_bin" => {
                let req: CreateBinRequest = serde_json::from_value(arguments)
                    .map_err(|e| McpError::InvalidParams(e.to_string()))?;
                self.media_tools.create_bin(req).await
                    .map(|msg| json!({"content": [{"type": "text", "text": msg}]}))
            }
            
            _ => Err(ResolveError::not_supported(format!("Tool '{}' not implemented", name))),
        };

        match result {
            Ok(response) => {
                debug!("Tool execution successful: {}", name);
                Ok(response)
            }
            Err(e) => {
                error!("Tool execution failed: {} - {}", name, e);
                Err(McpError::InternalError(e.to_string()))
            }
        }
    }

    /// Get JSON schema for a type
    fn get_schema_for_type<T>(&self) -> Value
    where
        T: schemars::JsonSchema,
    {
        let schema = schemars::schema_for!(T);
        serde_json::to_value(schema).unwrap_or_else(|_| json!({}))
    }
}

impl McpService for DaVinciResolveServer {
    async fn list_tools(&self) -> Result<Vec<Tool>, McpError> {
        Ok(self.get_tools().await)
    }

    async fn call_tool(&self, name: String, arguments: Option<Value>) -> Result<Value, McpError> {
        let args = arguments.unwrap_or_else(|| json!({}));
        self.call_tool(&name, args).await
    }

    async fn get_server_info(&self) -> Result<mcp::ServerInfo, McpError> {
        Ok(mcp::ServerInfo {
            name: self.config.server.name.clone(),
            version: self.config.server.version.clone(),
            instructions: self.config.server.instructions.clone(),
            capabilities: Some(self.get_capabilities()),
        })
    }
} 