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
            
            // Timeline Enhancement Tools (Phase 3 Week 2)
            Tool::new(
                "delete_timeline",
                "Delete a timeline by name",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The name of the timeline to delete"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_current_timeline",
                "Switch to a timeline by name",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The name of the timeline to set as current"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "create_empty_timeline",
                "Create a new timeline with custom settings",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name for the new timeline"
                        },
                        "frame_rate": {
                            "type": "string",
                            "description": "Optional frame rate (e.g. \"24\", \"29.97\", \"30\", \"60\")"
                        },
                        "resolution_width": {
                            "type": "integer",
                            "description": "Optional width in pixels (e.g. 1920)"
                        },
                        "resolution_height": {
                            "type": "integer",
                            "description": "Optional height in pixels (e.g. 1080)"
                        },
                        "start_timecode": {
                            "type": "string",
                            "description": "Optional start timecode (e.g. \"01:00:00:00\")"
                        },
                        "video_tracks": {
                            "type": "integer",
                            "description": "Optional number of video tracks (Default is project setting)"
                        },
                        "audio_tracks": {
                            "type": "integer",
                            "description": "Optional number of audio tracks (Default is project setting)"
                        }
                    },
                    "required": ["name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "add_clip_to_timeline",
                "Add a media pool clip to the timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip in the media pool"
                        },
                        "timeline_name": {
                            "type": "string",
                            "description": "Optional timeline to target (uses current if not specified)"
                        }
                    },
                    "required": ["clip_name"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "list_timelines_tool",
                "List all timelines in the current project as a tool",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_tracks",
                "Retrieve track information for a timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Optional timeline name (uses current if not specified)"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== PHASE 3 WEEK 3: COLOR OPERATIONS ====================

            Tool::new(
                "apply_lut",
                "Apply a LUT to a node in the color page",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "lut_path": {
                            "type": "string",
                            "description": "Path to the LUT file to apply"
                        },
                        "node_index": {
                            "type": "integer",
                            "description": "Index of the node to apply the LUT to (uses current node if None)"
                        }
                    },
                    "required": ["lut_path"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_color_wheel_param",
                "Set a color wheel parameter for a node",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "wheel": {
                            "type": "string",
                            "description": "Which color wheel to adjust",
                            "enum": ["lift", "gamma", "gain", "offset"]
                        },
                        "param": {
                            "type": "string", 
                            "description": "Which parameter to adjust",
                            "enum": ["red", "green", "blue", "master"]
                        },
                        "value": {
                            "type": "number",
                            "description": "The value to set (typically between -1.0 and 1.0)"
                        },
                        "node_index": {
                            "type": "integer",
                            "description": "Index of the node to set parameter for (uses current node if None)"
                        }
                    },
                    "required": ["wheel", "param", "value"]
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "add_node",
                "Add a new node to the current grade in the color page",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "node_type": {
                            "type": "string",
                            "description": "Type of node to add",
                            "enum": ["serial", "parallel", "layer"],
                            "default": "serial"
                        },
                        "label": {
                            "type": "string",
                            "description": "Optional label/name for the new node"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "copy_grade",
                "Copy a grade from one clip to another in the color page",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "source_clip_name": {
                            "type": "string",
                            "description": "Name of the source clip to copy grade from (uses current clip if None)"
                        },
                        "target_clip_name": {
                            "type": "string",
                            "description": "Name of the target clip to apply grade to (uses current clip if None)"
                        },
                        "mode": {
                            "type": "string",
                            "description": "What to copy",
                            "enum": ["full", "current_node", "all_nodes"],
                            "default": "full"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "save_color_preset",
                "Save a color preset from the specified clip",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to save preset from (uses current clip if None)"
                        },
                        "preset_name": {
                            "type": "string",
                            "description": "Name to give the preset (uses clip name if None)"
                        },
                        "album_name": {
                            "type": "string",
                            "description": "Album to save the preset to",
                            "default": "DaVinci Resolve"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "apply_color_preset",
                "Apply a color preset to the specified clip",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_id": {
                            "type": "string",
                            "description": "ID of the preset to apply (if known)"
                        },
                        "preset_name": {
                            "type": "string",
                            "description": "Name of the preset to apply (searches in album)"
                        },
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to apply preset to (uses current clip if None)"
                        },
                        "album_name": {
                            "type": "string",
                            "description": "Album containing the preset",
                            "default": "DaVinci Resolve"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_color_preset",
                "Delete a color preset",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_id": {
                            "type": "string",
                            "description": "ID of the preset to delete (if known)"
                        },
                        "preset_name": {
                            "type": "string",
                            "description": "Name of the preset to delete (searches in album)"
                        },
                        "album_name": {
                            "type": "string",
                            "description": "Album containing the preset",
                            "default": "DaVinci Resolve"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_lut",
                "Export a LUT from the current clip's grade",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to export grade from (uses current clip if None)"
                        },
                        "export_path": {
                            "type": "string",
                            "description": "Path to save the LUT file (generated if None)"
                        },
                        "lut_format": {
                            "type": "string",
                            "description": "Format of the LUT",
                            "enum": ["Cube", "Davinci", "3dl", "Panasonic"],
                            "default": "Cube"
                        },
                        "lut_size": {
                            "type": "string",
                            "description": "Size of the LUT",
                            "enum": ["17Point", "33Point", "65Point"],
                            "default": "33Point"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== PHASE 4 WEEK 1: TIMELINE ITEM MANIPULATION ====================

            Tool::new(
                "set_timeline_item_transform",
                "Set a transform property for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to modify"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "The name of the property to set",
                            "enum": ["Pan", "Tilt", "ZoomX", "ZoomY", "Rotation", "AnchorPointX", "AnchorPointY", "Pitch", "Yaw"]
                        },
                        "property_value": {
                            "type": "number",
                            "description": "The value to set for the property"
                        }
                    },
                    "required": ["timeline_item_id", "property_name", "property_value"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_item_crop",
                "Set a crop property for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to modify"
                        },
                        "crop_type": {
                            "type": "string",
                            "description": "The type of crop to set",
                            "enum": ["Left", "Right", "Top", "Bottom"]
                        },
                        "crop_value": {
                            "type": "number",
                            "description": "The value to set for the crop (0.0 to 1.0)",
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["timeline_item_id", "crop_type", "crop_value"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_item_composite",
                "Set composite properties for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to modify"
                        },
                        "composite_mode": {
                            "type": "string",
                            "description": "Optional composite mode to set",
                            "enum": ["Normal", "Add", "Multiply", "Screen", "Overlay", "SoftLight", "HardLight", "ColorDodge", "ColorBurn", "Darken", "Lighten", "Difference", "Exclusion"]
                        },
                        "opacity": {
                            "type": "number",
                            "description": "Optional opacity value to set (0.0 to 1.0)",
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_item_retime",
                "Set retiming properties for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to modify"
                        },
                        "speed": {
                            "type": "number",
                            "description": "Optional speed factor (e.g., 0.5 for 50%, 2.0 for 200%)",
                            "minimum": 0.0,
                            "maximum": 10.0
                        },
                        "process": {
                            "type": "string",
                            "description": "Optional retime process",
                            "enum": ["NearestFrame", "FrameBlend", "OpticalFlow"]
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_item_stabilization",
                "Set stabilization properties for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to modify"
                        },
                        "enabled": {
                            "type": "boolean",
                            "description": "Optional boolean to enable/disable stabilization"
                        },
                        "method": {
                            "type": "string",
                            "description": "Optional stabilization method",
                            "enum": ["Perspective", "Similarity", "Translation"]
                        },
                        "strength": {
                            "type": "number",
                            "description": "Optional strength value (0.0 to 1.0)",
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_item_audio",
                "Set audio properties for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to modify"
                        },
                        "volume": {
                            "type": "number",
                            "description": "Optional volume level (0.0 to 2.0, where 1.0 is unity gain)",
                            "minimum": 0.0,
                            "maximum": 2.0
                        },
                        "pan": {
                            "type": "number",
                            "description": "Optional pan value (-1.0 to 1.0, where -1.0 is left, 0 is center, 1.0 is right)",
                            "minimum": -1.0,
                            "maximum": 1.0
                        },
                        "eq_enabled": {
                            "type": "boolean",
                            "description": "Optional boolean to enable/disable EQ"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_item_properties",
                "Get all properties of a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to retrieve properties from"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "reset_timeline_item_properties",
                "Reset timeline item properties to default values",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to reset"
                        },
                        "property_type": {
                            "type": "string",
                            "description": "Optional property type to reset. If None, resets all properties",
                            "enum": ["transform", "crop", "composite", "retime", "stabilization", "audio"]
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
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
            instructions: Some("DaVinci Resolve MCP Server (Pure Rust) - Automate DaVinci Resolve workflows with 36 tools including project management, timeline operations, media pool management, timeline enhancement features, comprehensive color grading operations, and professional timeline item manipulation".to_string()),
        }
    }
}

impl Default for DaVinciResolveServer {
    fn default() -> Self {
        Self::new()
    }
} 