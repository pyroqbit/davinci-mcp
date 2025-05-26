use std::sync::{Arc, RwLock};
use serde_json::{json, Value};
use rmcp::{
    model::{CallToolResult, Content, ServerCapabilities, Implementation,
           InitializeResult, ProtocolVersion, ListToolsResult, CallToolRequestParam,
           ClientRequest, ClientNotification, ServerResult, Tool, ErrorData, CallToolRequestMethod},
    Service,
    service::{RoleServer, RequestContext},
};
use crate::{
    bridge::{ResolveBridge, ConnectionMode},
    config::Config,
    error::ResolveError,
    tools::{handle_tool_call},
};

/// Main DaVinci Resolve MCP Server
#[derive(Debug)]
pub struct DaVinciResolveServer {
    /// Configuration
    #[allow(dead_code)]
    config: Arc<Config>,
    /// Python bridge to DaVinci Resolve
    bridge: Arc<ResolveBridge>,
    /// Server initialized flag
    initialized: Arc<RwLock<bool>>,
}

impl DaVinciResolveServer {
    /// Create a new server instance with default configuration (simulation mode)
    pub fn new() -> Self {
        let config = Config::default();
        Self::with_config(config)
    }

    /// Create a new server instance with custom configuration (simulation mode)
    pub fn with_config(config: Config) -> Self {
        Self::with_mode_and_config(ConnectionMode::Simulation, config)
    }

    /// Create a new server instance with real DaVinci Resolve connection
    pub fn new_real() -> Self {
        let config = Config::default();
        Self::with_mode_and_config(ConnectionMode::Real, config)
    }

    /// Create a new server instance with specific connection mode and configuration
    pub fn with_mode_and_config(mode: ConnectionMode, config: Config) -> Self {
        let bridge = Arc::new(ResolveBridge::new(mode));
        Self {
            config: Arc::new(config),
            bridge,
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the server and DaVinci Resolve connection
    pub async fn initialize(&self) -> Result<(), ResolveError> {
        let mut initialized = self.initialized.write().unwrap();
        if *initialized {
            return Ok(());
        }

        // Initialize the bridge
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

            // Keyframe Animation Tools (Phase 4 Week 2)
            Tool::new(
                "add_keyframe",
                "Add a keyframe at the specified frame for a timeline item property",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item to add keyframe to"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "The name of the property to keyframe (e.g., 'Pan', 'ZoomX', 'Opacity')"
                        },
                        "frame": {
                            "type": "integer",
                            "description": "Frame position for the keyframe",
                            "minimum": 0
                        },
                        "value": {
                            "type": "number",
                            "description": "Value to set at the keyframe"
                        }
                    },
                    "required": ["timeline_item_id", "property_name", "frame", "value"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "modify_keyframe",
                "Modify an existing keyframe by changing its value or frame position",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "The name of the property with keyframe"
                        },
                        "frame": {
                            "type": "integer",
                            "description": "Current frame position of the keyframe to modify"
                        },
                        "new_value": {
                            "type": "number",
                            "description": "Optional new value for the keyframe"
                        },
                        "new_frame": {
                            "type": "integer",
                            "description": "Optional new frame position for the keyframe",
                            "minimum": 0
                        }
                    },
                    "required": ["timeline_item_id", "property_name", "frame"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_keyframe",
                "Delete a keyframe at the specified frame for a timeline item property",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "The name of the property with keyframe to delete"
                        },
                        "frame": {
                            "type": "integer",
                            "description": "Frame position of the keyframe to delete"
                        }
                    },
                    "required": ["timeline_item_id", "property_name", "frame"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_keyframe_interpolation",
                "Set the interpolation type for a keyframe",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "The name of the property with keyframe"
                        },
                        "frame": {
                            "type": "integer",
                            "description": "Frame position of the keyframe"
                        },
                        "interpolation_type": {
                            "type": "string",
                            "description": "Type of interpolation",
                            "enum": ["Linear", "Bezier", "Ease-In", "Ease-Out", "Hold"]
                        }
                    },
                    "required": ["timeline_item_id", "property_name", "frame", "interpolation_type"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "enable_keyframes",
                "Enable keyframe mode for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item"
                        },
                        "keyframe_mode": {
                            "type": "string",
                            "description": "Keyframe mode to enable",
                            "enum": ["All", "Color", "Sizing"],
                            "default": "All"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_keyframes",
                "Get keyframe information for a timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "The ID of the timeline item"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "Optional property name to get keyframes for (returns all if None)"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== PHASE 4 WEEK 3: RENDERING & DELIVERY OPERATIONS ====================

            Tool::new(
                "add_to_render_queue",
                "Add a timeline to the render queue with specified preset",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_name": {
                            "type": "string",
                            "description": "Name of the render preset to use"
                        },
                        "timeline_name": {
                            "type": "string",
                            "description": "Name of the timeline to render (uses current if None)"
                        },
                        "use_in_out_range": {
                            "type": "boolean",
                            "description": "Whether to render only the in/out range instead of entire timeline",
                            "default": false
                        }
                    },
                    "required": ["preset_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "start_render",
                "Start rendering all jobs in the render queue",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "clear_render_queue",
                "Clear all jobs from the render queue",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_render_status",
                "Get current render progress and status information",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_project",
                "Export project with metadata and optional media consolidation",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "export_path": {
                            "type": "string",
                            "description": "Path to export the project to"
                        },
                        "include_media": {
                            "type": "boolean",
                            "description": "Whether to include media files in export",
                            "default": false
                        },
                        "project_name": {
                            "type": "string",
                            "description": "Name of project to export (uses current project if None)"
                        }
                    },
                    "required": ["export_path"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "create_render_preset",
                "Create a custom render preset with specified settings",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_name": {
                            "type": "string",
                            "description": "Name for the render preset"
                        },
                        "format": {
                            "type": "string",
                            "description": "Output format",
                            "enum": ["MP4", "MOV", "MXF"]
                        },
                        "codec": {
                            "type": "string",
                            "description": "Video codec",
                            "enum": ["H.264", "H.265", "ProRes"]
                        },
                        "resolution_width": {
                            "type": "integer",
                            "description": "Width in pixels",
                            "minimum": 1920
                        },
                        "resolution_height": {
                            "type": "integer",
                            "description": "Height in pixels",
                            "minimum": 1080
                        },
                        "frame_rate": {
                            "type": "number",
                            "description": "Frame rate",
                            "minimum": 24.0,
                            "maximum": 60.0
                        },
                        "quality": {
                            "type": "integer",
                            "description": "Quality level (1-100)",
                            "minimum": 1,
                            "maximum": 100
                        },
                        "audio_codec": {
                            "type": "string",
                            "description": "Audio codec",
                            "enum": ["AAC", "ProRes"],
                            "default": "AAC"
                        },
                        "audio_bitrate": {
                            "type": "integer",
                            "description": "Audio bitrate in kbps",
                            "minimum": 64,
                            "maximum": 192,
                            "default": 192
                        }
                    },
                    "required": ["preset_name", "format", "codec", "resolution_width", "resolution_height", "frame_rate", "quality"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== PROJECT MANAGEMENT OPERATIONS ====================
            Tool::new(
                "save_project",
                "Save the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "close_project",
                "Close the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_project_setting",
                "Set a project setting to the specified value",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "setting_name": {
                            "type": "string",
                            "description": "The name of the setting to change"
                        },
                        "setting_value": {
                            "description": "The new value for the setting (can be string, integer, float, or boolean)"
                        }
                    },
                    "required": ["setting_name", "setting_value"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== AUDIO TRANSCRIPTION OPERATIONS ====================
            Tool::new(
                "transcribe_audio",
                "Transcribe audio for a clip",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to transcribe"
                        },
                        "language": {
                            "type": "string",
                            "description": "Language code for transcription (default: en-US)",
                            "default": "en-US"
                        }
                    },
                    "required": ["clip_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "clear_transcription",
                "Clear audio transcription for a clip",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to clear transcription from"
                        }
                    },
                    "required": ["clip_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== EXTENDED PROJECT MANAGEMENT OPERATIONS ====================
            Tool::new(
                "delete_media",
                "Delete a media clip from the media pool by name",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to delete"
                        }
                    },
                    "required": ["clip_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "move_media_to_bin",
                "Move a media clip to a specific bin in the media pool",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_name": {
                            "type": "string",
                            "description": "Name of the clip to move"
                        },
                        "bin_name": {
                            "type": "string",
                            "description": "Name of the target bin"
                        }
                    },
                    "required": ["clip_name", "bin_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_folder",
                "Export a folder to a DRB file or other format",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "folder_name": {
                            "type": "string",
                            "description": "Name of the folder to export"
                        },
                        "export_path": {
                            "type": "string",
                            "description": "Path to save the exported file"
                        },
                        "export_type": {
                            "type": "string",
                            "description": "Export format (DRB is default and currently the only supported option)",
                            "default": "DRB"
                        }
                    },
                    "required": ["folder_name", "export_path"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "transcribe_folder_audio",
                "Transcribe audio for all clips in a folder",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "folder_name": {
                            "type": "string",
                            "description": "Name of the folder containing clips to transcribe"
                        },
                        "language": {
                            "type": "string",
                            "description": "Language code for transcription (default: en-US)",
                            "default": "en-US"
                        }
                    },
                    "required": ["folder_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "clear_folder_transcription",
                "Clear audio transcription for all clips in a folder",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "folder_name": {
                            "type": "string",
                            "description": "Name of the folder to clear transcriptions from"
                        }
                    },
                    "required": ["folder_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== CACHE AND OPTIMIZATION OPERATIONS ====================
            Tool::new(
                "set_cache_mode",
                "Set cache mode for the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "mode": {
                            "type": "string",
                            "description": "Cache mode to set",
                            "enum": ["auto", "on", "off"]
                        }
                    },
                    "required": ["mode"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_optimized_media_mode",
                "Set optimized media mode for the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "mode": {
                            "type": "string",
                            "description": "Optimized media mode to set",
                            "enum": ["auto", "on", "off"]
                        }
                    },
                    "required": ["mode"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_proxy_mode",
                "Set proxy media mode for the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "mode": {
                            "type": "string",
                            "description": "Proxy mode to set",
                            "enum": ["auto", "on", "off"]
                        }
                    },
                    "required": ["mode"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_proxy_quality",
                "Set proxy media quality for the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "quality": {
                            "type": "string",
                            "description": "Proxy quality to set",
                            "enum": ["quarter", "half", "threeQuarter", "full"]
                        }
                    },
                    "required": ["quality"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_cache_path",
                "Set cache file path for the current project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "path_type": {
                            "type": "string",
                            "description": "Type of cache path to set",
                            "enum": ["local", "network"]
                        },
                        "path": {
                            "type": "string",
                            "description": "File system path for the cache"
                        }
                    },
                    "required": ["path_type", "path"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "generate_optimized_media",
                "Generate optimized media for specified clips or all clips if none specified",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_names": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Optional list of clip names. If None, processes all clips in media pool"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_optimized_media",
                "Delete optimized media for specified clips or all clips if none specified",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "clip_names": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "Optional list of clip names. If None, processes all clips in media pool"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== EXTENDED COLOR OPERATIONS ====================
            Tool::new(
                "create_color_preset_album",
                "Create a new album for color presets",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "album_name": {
                            "type": "string",
                            "description": "Name for the new album"
                        }
                    },
                    "required": ["album_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_color_preset_album",
                "Delete a color preset album",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "album_name": {
                            "type": "string",
                            "description": "Name of the album to delete"
                        }
                    },
                    "required": ["album_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_all_power_grade_luts",
                "Export all PowerGrade presets as LUT files",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "export_dir": {
                            "type": "string",
                            "description": "Directory to save the exported LUTs"
                        }
                    },
                    "required": ["export_dir"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== LAYOUT AND INTERFACE MANAGEMENT ====================
            Tool::new(
                "save_layout_preset",
                "Save the current UI layout as a preset",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_name": {
                            "type": "string",
                            "description": "Name for the saved preset"
                        }
                    },
                    "required": ["preset_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "load_layout_preset",
                "Load a UI layout preset",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_name": {
                            "type": "string",
                            "description": "Name of the preset to load"
                        }
                    },
                    "required": ["preset_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_layout_preset",
                "Export a layout preset to a file",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_name": {
                            "type": "string",
                            "description": "Name of the preset to export"
                        },
                        "export_path": {
                            "type": "string",
                            "description": "Path to export the preset file to"
                        }
                    },
                    "required": ["preset_name", "export_path"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "import_layout_preset",
                "Import a layout preset from a file",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "import_path": {
                            "type": "string",
                            "description": "Path to the preset file to import"
                        },
                        "preset_name": {
                            "type": "string",
                            "description": "Name to save the imported preset as (uses filename if None)"
                        }
                    },
                    "required": ["import_path"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_layout_preset",
                "Delete a layout preset",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "preset_name": {
                            "type": "string",
                            "description": "Name of the preset to delete"
                        }
                    },
                    "required": ["preset_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== APPLICATION CONTROL ====================
            Tool::new(
                "quit_app",
                "Quit DaVinci Resolve application",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "force": {
                            "type": "boolean",
                            "description": "Whether to force quit even if unsaved changes (potentially dangerous)",
                            "default": false
                        },
                        "save_project": {
                            "type": "boolean",
                            "description": "Whether to save the project before quitting",
                            "default": true
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "restart_app",
                "Restart DaVinci Resolve application",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "wait_seconds": {
                            "type": "integer",
                            "description": "Seconds to wait between quit and restart",
                            "default": 5
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "open_settings",
                "Open the Project Settings dialog in DaVinci Resolve",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "open_app_preferences",
                "Open the Preferences dialog in DaVinci Resolve",
                Arc::new(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== CLOUD OPERATIONS ====================
            Tool::new(
                "create_cloud_project",
                "Create a new cloud project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "project_name": {
                            "type": "string",
                            "description": "Name for the new cloud project"
                        },
                        "folder_path": {
                            "type": "string",
                            "description": "Optional path for the cloud project folder"
                        }
                    },
                    "required": ["project_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "import_cloud_project",
                "Import a project from DaVinci Resolve cloud",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "cloud_id": {
                            "type": "string",
                            "description": "Cloud ID or reference of the project to import"
                        },
                        "project_name": {
                            "type": "string",
                            "description": "Optional custom name for the imported project (uses original name if None)"
                        }
                    },
                    "required": ["cloud_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "restore_cloud_project",
                "Restore a project from DaVinci Resolve cloud",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "cloud_id": {
                            "type": "string",
                            "description": "Cloud ID or reference of the project to restore"
                        },
                        "project_name": {
                            "type": "string",
                            "description": "Optional custom name for the restored project (uses original name if None)"
                        }
                    },
                    "required": ["cloud_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_project_to_cloud",
                "Export current or specified project to DaVinci Resolve cloud",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "project_name": {
                            "type": "string",
                            "description": "Optional name of project to export (uses current project if None)"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "add_user_to_cloud_project",
                "Add a user to a cloud project with specified permissions",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "cloud_id": {
                            "type": "string",
                            "description": "Cloud ID of the project"
                        },
                        "user_email": {
                            "type": "string",
                            "description": "Email of the user to add"
                        },
                        "permissions": {
                            "type": "string",
                            "description": "Permission level",
                            "enum": ["viewer", "editor", "admin"],
                            "default": "viewer"
                        }
                    },
                    "required": ["cloud_id", "user_email"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "remove_user_from_cloud_project",
                "Remove a user from a cloud project",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "cloud_id": {
                            "type": "string",
                            "description": "Cloud ID of the project"
                        },
                        "user_email": {
                            "type": "string",
                            "description": "Email of the user to remove"
                        }
                    },
                    "required": ["cloud_id", "user_email"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== OBJECT INSPECTION ====================
            Tool::new(
                "object_help",
                "Get human-readable help for a DaVinci Resolve API object",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "object_type": {
                            "type": "string",
                            "description": "Type of object to get help for",
                            "enum": ["resolve", "project_manager", "project", "media_pool", "timeline", "media_storage"]
                        }
                    },
                    "required": ["object_type"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "inspect_custom_object",
                "Inspect a custom DaVinci Resolve API object by path",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "object_path": {
                            "type": "string",
                            "description": "Path to the object using dot notation (e.g., 'resolve.GetMediaStorage()')"
                        }
                    },
                    "required": ["object_path"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== PROJECT PROPERTIES ====================
            Tool::new(
                "set_project_property",
                "Set a project property value",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "property_name": {
                            "type": "string",
                            "description": "Name of the property to set"
                        },
                        "property_value": {
                            "description": "Value to set for the property"
                        }
                    },
                    "required": ["property_name", "property_value"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_format",
                "Set timeline format (resolution and frame rate)",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "width": {
                            "type": "integer",
                            "description": "Timeline width in pixels"
                        },
                        "height": {
                            "type": "integer",
                            "description": "Timeline height in pixels"
                        },
                        "frame_rate": {
                            "type": "number",
                            "description": "Timeline frame rate"
                        },
                        "interlaced": {
                            "type": "boolean",
                            "description": "Whether the timeline should use interlaced processing",
                            "default": false
                        }
                    },
                    "required": ["width", "height", "frame_rate"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== TIMELINE OBJECT API ====================
            Tool::new(
                "get_timeline_name",
                "Get timeline name",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name to get (uses current if None)"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_name",
                "Set timeline name",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name to set"
                        },
                        "new_name": {
                            "type": "string",
                            "description": "New name for the timeline"
                        }
                    },
                    "required": ["timeline_name", "new_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_frames",
                "Get timeline frame information",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_timecode",
                "Set timeline timecode",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "timecode": {
                            "type": "string",
                            "description": "Timecode to set"
                        }
                    },
                    "required": ["timecode"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_track_count",
                "Get timeline track count",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "track_type": {
                            "type": "string",
                            "description": "Track type",
                            "enum": ["video", "audio", "subtitle"]
                        }
                    },
                    "required": ["track_type"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_items_in_track",
                "Get items in timeline track",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "track_type": {
                            "type": "string",
                            "description": "Track type",
                            "enum": ["video", "audio", "subtitle"]
                        },
                        "track_index": {
                            "type": "integer",
                            "description": "Track index"
                        }
                    },
                    "required": ["track_type", "track_index"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "add_timeline_marker",
                "Add marker to timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "frame_id": {
                            "type": "number",
                            "description": "Frame ID for the marker"
                        },
                        "color": {
                            "type": "string",
                            "description": "Marker color",
                            "default": "Blue"
                        },
                        "name": {
                            "type": "string",
                            "description": "Marker name",
                            "default": ""
                        },
                        "note": {
                            "type": "string",
                            "description": "Marker note",
                            "default": ""
                        },
                        "duration": {
                            "type": "number",
                            "description": "Marker duration",
                            "default": 1.0
                        },
                        "custom_data": {
                            "type": "string",
                            "description": "Custom data",
                            "default": ""
                        }
                    },
                    "required": ["frame_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_markers",
                "Get timeline markers",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_timeline_marker",
                "Delete timeline marker",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "frame_num": {
                            "type": "number",
                            "description": "Frame number"
                        },
                        "color": {
                            "type": "string",
                            "description": "Marker color to delete"
                        },
                        "custom_data": {
                            "type": "string",
                            "description": "Custom data to match"
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "duplicate_timeline",
                "Duplicate timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "source_timeline_name": {
                            "type": "string",
                            "description": "Source timeline name"
                        },
                        "new_timeline_name": {
                            "type": "string",
                            "description": "New timeline name"
                        }
                    },
                    "required": ["source_timeline_name", "new_timeline_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "create_compound_clip",
                "Create compound clip from timeline items",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "timeline_item_ids": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Timeline item IDs to include"
                        },
                        "clip_name": {
                            "type": "string",
                            "description": "Compound clip name"
                        }
                    },
                    "required": ["timeline_item_ids", "clip_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "create_fusion_clip",
                "Create Fusion clip from timeline items",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "timeline_item_ids": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Timeline item IDs to include"
                        }
                    },
                    "required": ["timeline_item_ids"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "export_timeline",
                "Export timeline to file",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "file_name": {
                            "type": "string",
                            "description": "Export file name"
                        },
                        "export_type": {
                            "type": "string",
                            "description": "Export type",
                            "enum": ["AAF", "EDL", "XML", "FCPXML", "DRT", "ADL", "OTIO"]
                        },
                        "export_subtype": {
                            "type": "string",
                            "description": "Export subtype"
                        }
                    },
                    "required": ["file_name", "export_type"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "insert_generator",
                "Insert generator into timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "generator_name": {
                            "type": "string",
                            "description": "Generator name"
                        },
                        "generator_type": {
                            "type": "string",
                            "description": "Generator type",
                            "enum": ["standard", "fusion", "ofx"],
                            "default": "standard"
                        }
                    },
                    "required": ["generator_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "insert_title",
                "Insert title into timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "title_name": {
                            "type": "string",
                            "description": "Title name"
                        },
                        "title_type": {
                            "type": "string",
                            "description": "Title type",
                            "enum": ["standard", "fusion"],
                            "default": "standard"
                        }
                    },
                    "required": ["title_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "grab_still",
                "Grab still from timeline",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_name": {
                            "type": "string",
                            "description": "Timeline name (uses current if None)"
                        },
                        "still_frame_source": {
                            "type": "string",
                            "description": "Still frame source"
                        },
                        "grab_all": {
                            "type": "boolean",
                            "description": "Grab all stills",
                            "default": false
                        }
                    },
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),

            // ==================== TIMELINE ITEM OBJECT API ====================
            Tool::new(
                "get_timeline_item_property",
                "Get timeline item property",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "property_key": {
                            "type": "string",
                            "description": "Property key (optional - returns all if not specified)"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_timeline_item_property",
                "Set timeline item property",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "property_key": {
                            "type": "string",
                            "description": "Property key"
                        },
                        "property_value": {
                            "description": "Property value"
                        }
                    },
                    "required": ["timeline_item_id", "property_key", "property_value"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_item_details",
                "Get timeline item details",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "add_timeline_item_marker",
                "Add marker to timeline item",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "frame_id": {
                            "type": "number",
                            "description": "Frame ID for the marker"
                        },
                        "color": {
                            "type": "string",
                            "description": "Marker color",
                            "default": "Blue"
                        },
                        "name": {
                            "type": "string",
                            "description": "Marker name",
                            "default": ""
                        },
                        "note": {
                            "type": "string",
                            "description": "Marker note",
                            "default": ""
                        },
                        "duration": {
                            "type": "number",
                            "description": "Marker duration",
                            "default": 1.0
                        },
                        "custom_data": {
                            "type": "string",
                            "description": "Custom data",
                            "default": ""
                        }
                    },
                    "required": ["timeline_item_id", "frame_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "get_timeline_item_markers",
                "Get timeline item markers",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "delete_timeline_item_marker",
                "Delete timeline item marker",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "frame_num": {
                            "type": "number",
                            "description": "Frame number"
                        },
                        "color": {
                            "type": "string",
                            "description": "Marker color to delete"
                        },
                        "custom_data": {
                            "type": "string",
                            "description": "Custom data to match"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "timeline_item_flag",
                "Manage timeline item flags",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "color": {
                            "type": "string",
                            "description": "Flag color (optional - returns all flags if not specified)"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "timeline_item_color",
                "Manage timeline item color",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "color_name": {
                            "type": "string",
                            "description": "Color name (optional - returns current color if not specified)"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "fusion_comp",
                "Manage Fusion compositions",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "comp_index": {
                            "type": "integer",
                            "description": "Composition index"
                        },
                        "comp_name": {
                            "type": "string",
                            "description": "Composition name"
                        },
                        "file_path": {
                            "type": "string",
                            "description": "File path for import/export"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "version",
                "Manage timeline item versions",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "version_name": {
                            "type": "string",
                            "description": "Version name"
                        },
                        "version_type": {
                            "type": "string",
                            "description": "Version type",
                            "enum": ["local", "remote"],
                            "default": "local"
                        },
                        "new_version_name": {
                            "type": "string",
                            "description": "New version name for rename"
                        }
                    },
                    "required": ["timeline_item_id", "version_name"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "stereo_params",
                "Manage stereo parameters",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "params": {
                            "description": "Stereo parameters"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "node_lut",
                "Manage node LUT",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "node_index": {
                            "type": "integer",
                            "description": "Node index"
                        },
                        "lut_path": {
                            "type": "string",
                            "description": "LUT file path (optional - returns current LUT if not specified)"
                        }
                    },
                    "required": ["timeline_item_id", "node_index"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "set_cdl",
                "Set CDL parameters",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "cdl_map": {
                            "description": "CDL parameters"
                        }
                    },
                    "required": ["timeline_item_id", "cdl_map"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "take",
                "Manage timeline item takes",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "timeline_item_id": {
                            "type": "string",
                            "description": "Timeline item ID"
                        },
                        "media_pool_item": {
                            "type": "string",
                            "description": "Media pool item for new take"
                        },
                        "start_frame": {
                            "type": "number",
                            "description": "Start frame"
                        },
                        "end_frame": {
                            "type": "number",
                            "description": "End frame"
                        },
                        "take_index": {
                            "type": "integer",
                            "description": "Take index"
                        }
                    },
                    "required": ["timeline_item_id"],
                    "additionalProperties": false
                }).as_object().unwrap().clone()),
            ),
            Tool::new(
                "copy_grades",
                "Copy grades between timeline items",
                Arc::new(json!({
                    "type": "object",
                    "properties": {
                        "source_timeline_item_id": {
                            "type": "string",
                            "description": "Source timeline item ID"
                        },
                        "target_timeline_item_ids": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Target timeline item IDs"
                        }
                    },
                    "required": ["source_timeline_item_id", "target_timeline_item_ids"],
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
    ) -> Result<ServerResult, ErrorData> {
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
                Err(ErrorData::method_not_found::<CallToolRequestMethod>())
            }
        }
    }

    async fn handle_notification(
        &self,
        _notification: ClientNotification,
    ) -> Result<(), ErrorData> {
        // Handle notifications if needed
        Ok(())
    }

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability {
                    list_changed: None,
                }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "davinci-resolve-mcp".into(),
                version: "2.0.0".into(),
            },
            instructions: Some("DaVinci Resolve MCP Server (Pure Rust) - Automate DaVinci Resolve workflows with 120+ tools including project management, timeline operations, media pool management, timeline enhancement features, comprehensive color grading operations, professional timeline item manipulation, render & delivery operations, audio transcription, cache optimization, layout management, cloud operations, application control, complete Timeline API, and comprehensive TimelineItem API".to_string()),
        }
    }
}

impl Default for DaVinciResolveServer {
    fn default() -> Self {
        Self::new()
    }
} 