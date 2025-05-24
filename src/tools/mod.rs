use std::sync::Arc;
use serde::Deserialize;
use schemars::JsonSchema;

use crate::bridge::ResolveBridge;
use crate::error::ResolveResult;

// Helper function for default color value
fn default_color() -> String {
    "Blue".to_string()
}

// Helper function for default sync method
fn default_sync_method() -> String {
    "waveform".to_string()
}

// ============================================
// REQUEST TYPES FOR ALL TOOLS
// ============================================

// ---- Existing Phase 1 & 2 Request Types ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateProjectRequest {
    #[schemars(description = "Name for the new project")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OpenProjectRequest {
    #[schemars(description = "Name of the project to open")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SwitchPageRequest {
    #[schemars(description = "The page to switch to. Options: 'media', 'cut', 'edit', 'fusion', 'color', 'fairlight', 'deliver'")]
    pub page: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTimelineRequest {
    #[schemars(description = "Name for the new timeline")]
    pub name: String,
    #[schemars(description = "Optional frame rate (e.g. '24', '29.97', '30', '60')")]
    pub frame_rate: Option<String>,
    #[schemars(description = "Optional width in pixels (e.g. 1920)")]
    pub resolution_width: Option<i32>,
    #[schemars(description = "Optional height in pixels (e.g. 1080)")]
    pub resolution_height: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportMediaRequest {
    #[schemars(description = "Path to the media file to import")]
    pub file_path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddMarkerRequest {
    #[schemars(description = "Frame number to add the marker at (defaults to current position if None)")]
    pub frame: Option<i32>,
    #[schemars(description = "Marker color (Blue, Cyan, Green, Yellow, Red, Pink, Purple, Fuchsia, Rose, Lavender, Sky, Mint, Lemon, Sand, Cocoa, Cream)")]
    #[serde(default = "default_color")]
    pub color: String,
    #[schemars(description = "Text note to add to the marker")]
    pub note: String,
}

// ---- Phase 3 Week 1: Media Operations Request Types ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateBinRequest {
    #[schemars(description = "Name for the new bin")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AutoSyncAudioRequest {
    #[schemars(description = "List of clip names to sync")]
    pub clip_names: Vec<String>,
    #[schemars(description = "Method to use for synchronization - 'waveform' or 'timecode'")]
    #[serde(default = "default_sync_method")]
    pub sync_method: String,
    #[schemars(description = "Whether to append the audio or replace it")]
    #[serde(default)]
    pub append_mode: bool,
    #[schemars(description = "Optional bin to move synchronized clips to")]
    pub target_bin: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnlinkClipsRequest {
    #[schemars(description = "List of clip names to unlink")]
    pub clip_names: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RelinkClipsRequest {
    #[schemars(description = "List of clip names to relink")]
    pub clip_names: Vec<String>,
    #[schemars(description = "Optional list of specific media file paths to use for relinking")]
    pub media_paths: Option<Vec<String>>,
    #[schemars(description = "Optional folder path to search for media files")]
    pub folder_path: Option<String>,
    #[schemars(description = "Whether to search the folder path recursively")]
    #[serde(default)]
    pub recursive: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateSubClipRequest {
    #[schemars(description = "Name of the source clip")]
    pub clip_name: String,
    #[schemars(description = "Start frame (in point)")]
    pub start_frame: i32,
    #[schemars(description = "End frame (out point)")]
    pub end_frame: i32,
    #[schemars(description = "Optional name for the subclip (defaults to original name with '_subclip')")]
    pub sub_clip_name: Option<String>,
    #[schemars(description = "Optional bin to place the subclip in")]
    pub bin_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkProxyMediaRequest {
    #[schemars(description = "Name of the clip to link proxy to")]
    pub clip_name: String,
    #[schemars(description = "Path to the proxy media file")]
    pub proxy_file_path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnlinkProxyMediaRequest {
    #[schemars(description = "Name of the clip to unlink proxy from")]
    pub clip_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReplaceClipRequest {
    #[schemars(description = "Name of the clip to be replaced")]
    pub clip_name: String,
    #[schemars(description = "Path to the replacement media file")]
    pub replacement_path: String,
}

// Timeline Enhancement Tools (Phase 3 Week 2)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTimelineRequest {
    #[schemars(description = "Name of the timeline to delete")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCurrentTimelineRequest {
    #[schemars(description = "Name of the timeline to set as current")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateEmptyTimelineRequest {
    #[schemars(description = "Name for the new timeline")]
    pub name: String,
    #[schemars(description = "Optional frame rate (e.g. '24', '29.97', '30', '60')")]
    pub frame_rate: Option<String>,
    #[schemars(description = "Optional width in pixels (e.g. 1920)")]
    pub resolution_width: Option<i32>,
    #[schemars(description = "Optional height in pixels (e.g. 1080)")]
    pub resolution_height: Option<i32>,
    #[schemars(description = "Optional start timecode (e.g. '01:00:00:00')")]
    pub start_timecode: Option<String>,
    #[schemars(description = "Optional number of video tracks (Default is project setting)")]
    pub video_tracks: Option<i32>,
    #[schemars(description = "Optional number of audio tracks (Default is project setting)")]
    pub audio_tracks: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddClipToTimelineRequest {
    #[schemars(description = "Name of the clip in the media pool")]
    pub clip_name: String,
    #[schemars(description = "Optional timeline to target (uses current if not specified)")]
    pub timeline_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineTracksRequest {
    #[schemars(description = "Optional timeline name (uses current if not specified)")]
    pub timeline_name: Option<String>,
}

// ---- Color Operations Request Types (Phase 3 Week 3) ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApplyLutRequest {
    #[schemars(description = "Path to the LUT file to apply")]
    pub lut_path: String,
    #[schemars(description = "Index of the node to apply the LUT to (uses current node if None)")]
    pub node_index: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetColorWheelParamRequest {
    #[schemars(description = "Which color wheel to adjust ('lift', 'gamma', 'gain', 'offset')")]
    pub wheel: String,
    #[schemars(description = "Which parameter to adjust ('red', 'green', 'blue', 'master')")]
    pub param: String,
    #[schemars(description = "The value to set (typically between -1.0 and 1.0)")]
    pub value: f64,
    #[schemars(description = "Index of the node to set parameter for (uses current node if None)")]
    pub node_index: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddNodeRequest {
    #[schemars(description = "Type of node to add. Options: 'serial', 'parallel', 'layer'")]
    #[serde(default = "default_node_type")]
    pub node_type: String,
    #[schemars(description = "Optional label/name for the new node")]
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CopyGradeRequest {
    #[schemars(description = "Name of the source clip to copy grade from (uses current clip if None)")]
    pub source_clip_name: Option<String>,
    #[schemars(description = "Name of the target clip to apply grade to (uses current clip if None)")]
    pub target_clip_name: Option<String>,
    #[schemars(description = "What to copy - 'full' (entire grade), 'current_node', or 'all_nodes'")]
    #[serde(default = "default_copy_mode")]
    pub mode: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SaveColorPresetRequest {
    #[schemars(description = "Name of the clip to save preset from (uses current clip if None)")]
    pub clip_name: Option<String>,
    #[schemars(description = "Name to give the preset (uses clip name if None)")]
    pub preset_name: Option<String>,
    #[schemars(description = "Album to save the preset to")]
    #[serde(default = "default_album")]
    pub album_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApplyColorPresetRequest {
    #[schemars(description = "ID of the preset to apply (if known)")]
    pub preset_id: Option<String>,
    #[schemars(description = "Name of the preset to apply (searches in album)")]
    pub preset_name: Option<String>,
    #[schemars(description = "Name of the clip to apply preset to (uses current clip if None)")]
    pub clip_name: Option<String>,
    #[schemars(description = "Album containing the preset")]
    #[serde(default = "default_album")]
    pub album_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteColorPresetRequest {
    #[schemars(description = "ID of the preset to delete (if known)")]
    pub preset_id: Option<String>,
    #[schemars(description = "Name of the preset to delete (searches in album)")]
    pub preset_name: Option<String>,
    #[schemars(description = "Album containing the preset")]
    #[serde(default = "default_album")]
    pub album_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportLutRequest {
    #[schemars(description = "Name of the clip to export grade from (uses current clip if None)")]
    pub clip_name: Option<String>,
    #[schemars(description = "Path to save the LUT file (generated if None)")]
    pub export_path: Option<String>,
    #[schemars(description = "Format of the LUT. Options: 'Cube', 'Davinci', '3dl', 'Panasonic'")]
    #[serde(default = "default_lut_format")]
    pub lut_format: String,
    #[schemars(description = "Size of the LUT. Options: '17Point', '33Point', '65Point'")]
    #[serde(default = "default_lut_size")]
    pub lut_size: String,
}

// ---- Timeline Item Operations Request Types (Phase 4 Week 1) ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemTransformRequest {
    #[schemars(description = "The ID of the timeline item to modify")]
    pub timeline_item_id: String,
    #[schemars(description = "The name of the property to set. Options: 'Pan', 'Tilt', 'ZoomX', 'ZoomY', 'Rotation', 'AnchorPointX', 'AnchorPointY', 'Pitch', 'Yaw'")]
    pub property_name: String,
    #[schemars(description = "The value to set for the property")]
    pub property_value: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemCropRequest {
    #[schemars(description = "The ID of the timeline item to modify")]
    pub timeline_item_id: String,
    #[schemars(description = "The type of crop to set. Options: 'Left', 'Right', 'Top', 'Bottom'")]
    pub crop_type: String,
    #[schemars(description = "The value to set for the crop (0.0 to 1.0)")]
    pub crop_value: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemCompositeRequest {
    #[schemars(description = "The ID of the timeline item to modify")]
    pub timeline_item_id: String,
    #[schemars(description = "Optional composite mode to set (e.g., 'Normal', 'Add', 'Multiply')")]
    pub composite_mode: Option<String>,
    #[schemars(description = "Optional opacity value to set (0.0 to 1.0)")]
    pub opacity: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemRetimeRequest {
    #[schemars(description = "The ID of the timeline item to modify")]
    pub timeline_item_id: String,
    #[schemars(description = "Optional speed factor (e.g., 0.5 for 50%, 2.0 for 200%)")]
    pub speed: Option<f64>,
    #[schemars(description = "Optional retime process. Options: 'NearestFrame', 'FrameBlend', 'OpticalFlow'")]
    pub process: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemStabilizationRequest {
    #[schemars(description = "The ID of the timeline item to modify")]
    pub timeline_item_id: String,
    #[schemars(description = "Optional boolean to enable/disable stabilization")]
    pub enabled: Option<bool>,
    #[schemars(description = "Optional stabilization method. Options: 'Perspective', 'Similarity', 'Translation'")]
    pub method: Option<String>,
    #[schemars(description = "Optional strength value (0.0 to 1.0)")]
    pub strength: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemAudioRequest {
    #[schemars(description = "The ID of the timeline item to modify")]
    pub timeline_item_id: String,
    #[schemars(description = "Optional volume level (0.0 to 2.0, where 1.0 is unity gain)")]
    pub volume: Option<f64>,
    #[schemars(description = "Optional pan value (-1.0 to 1.0, where -1.0 is left, 0 is center, 1.0 is right)")]
    pub pan: Option<f64>,
    #[schemars(description = "Optional boolean to enable/disable EQ")]
    pub eq_enabled: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineItemPropertiesRequest {
    #[schemars(description = "The ID of the timeline item to retrieve properties from")]
    pub timeline_item_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResetTimelineItemPropertiesRequest {
    #[schemars(description = "The ID of the timeline item to reset")]
    pub timeline_item_id: String,
    #[schemars(description = "Optional property type to reset. Options: 'transform', 'crop', 'composite', 'retime', 'stabilization', 'audio'. If None, resets all properties")]
    pub property_type: Option<String>,
}

// Helper functions for color operations defaults
fn default_node_type() -> String {
    "serial".to_string()
}

fn default_copy_mode() -> String {
    "full".to_string()
}

fn default_album() -> String {
    "DaVinci Resolve".to_string()
}

fn default_lut_format() -> String {
    "Cube".to_string()
}

fn default_lut_size() -> String {
    "33Point".to_string()
}

// ============================================
// TOOL IMPLEMENTATIONS
// ============================================

#[derive(Debug)]
pub struct ProjectTools {
    bridge: Arc<ResolveBridge>,
}

impl ProjectTools {
    pub fn new(bridge: Arc<ResolveBridge>) -> Self {
        Self { bridge }
    }

    pub async fn create_project(&self, req: CreateProjectRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "name": req.name
        });
        
        self.bridge.call_api("create_project", args).await?;
        Ok(format!("Successfully created project '{}'", req.name))
    }

    pub async fn open_project(&self, req: OpenProjectRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "name": req.name
        });
        
        self.bridge.call_api("open_project", args).await?;
        Ok(format!("Successfully opened project '{}'", req.name))
    }

    pub async fn switch_page(&self, req: SwitchPageRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "page": req.page
        });
        
        self.bridge.call_api("switch_page", args).await?;
        Ok(format!("Successfully switched to {} page", req.page))
    }
}

#[derive(Debug)]
pub struct TimelineTools {
    bridge: Arc<ResolveBridge>,
}

impl TimelineTools {
    pub fn new(bridge: Arc<ResolveBridge>) -> Self {
        Self { bridge }
    }

    pub async fn create_timeline(&self, req: CreateTimelineRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "name": req.name,
            "frame_rate": req.frame_rate,
            "resolution_width": req.resolution_width,
            "resolution_height": req.resolution_height,
        });
        
        self.bridge.call_api("create_timeline", args).await?;
        Ok(format!("Successfully created timeline '{}'", req.name))
    }

    pub async fn add_marker(&self, req: AddMarkerRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "frame": req.frame,
            "color": req.color,
            "note": req.note,
        });
        
        self.bridge.call_api("add_marker", args).await?;
        let frame_info = req.frame.map(|f| format!(" at frame {}", f)).unwrap_or_default();
        Ok(format!("Successfully added {} marker{} with note: '{}'", req.color, frame_info, req.note))
    }
}

#[derive(Debug)]
pub struct MediaTools {
    bridge: Arc<ResolveBridge>,
}

impl MediaTools {
    pub fn new(bridge: Arc<ResolveBridge>) -> Self {
        Self { bridge }
    }

    pub async fn import_media(&self, req: ImportMediaRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "file_path": req.file_path
        });
        
        self.bridge.call_api("import_media", args).await?;
        Ok(format!("Successfully imported media: {}", req.file_path))
    }

    // ---- Phase 3 Week 1: New Media Operations ----

    pub async fn create_bin(&self, req: CreateBinRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "name": req.name
        });
        
        self.bridge.call_api("create_bin", args).await?;
        Ok(format!("Successfully created bin '{}'", req.name))
    }

    pub async fn auto_sync_audio(&self, req: AutoSyncAudioRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_names": req.clip_names,
            "sync_method": req.sync_method,
            "append_mode": req.append_mode,
            "target_bin": req.target_bin
        });
        
        self.bridge.call_api("auto_sync_audio", args).await?;
        Ok(format!("Successfully synchronized {} clips using {} method", 
                  req.clip_names.len(), req.sync_method))
    }

    pub async fn unlink_clips(&self, req: UnlinkClipsRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_names": req.clip_names
        });
        
        self.bridge.call_api("unlink_clips", args).await?;
        Ok(format!("Successfully unlinked {} clips", req.clip_names.len()))
    }

    pub async fn relink_clips(&self, req: RelinkClipsRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_names": req.clip_names,
            "media_paths": req.media_paths,
            "folder_path": req.folder_path,
            "recursive": req.recursive
        });
        
        self.bridge.call_api("relink_clips", args).await?;
        Ok(format!("Successfully relinked {} clips", req.clip_names.len()))
    }

    pub async fn create_sub_clip(&self, req: CreateSubClipRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_name": req.clip_name,
            "start_frame": req.start_frame,
            "end_frame": req.end_frame,
            "sub_clip_name": req.sub_clip_name,
            "bin_name": req.bin_name
        });
        
        self.bridge.call_api("create_sub_clip", args).await?;
        let sub_name = req.sub_clip_name.unwrap_or_else(|| format!("{}_subclip", req.clip_name));
        Ok(format!("Successfully created subclip '{}' from '{}' (frames {}-{})", 
                  sub_name, req.clip_name, req.start_frame, req.end_frame))
    }

    pub async fn link_proxy_media(&self, req: LinkProxyMediaRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_name": req.clip_name,
            "proxy_file_path": req.proxy_file_path
        });
        
        self.bridge.call_api("link_proxy_media", args).await?;
        Ok(format!("Successfully linked proxy media for clip '{}'", req.clip_name))
    }

    pub async fn unlink_proxy_media(&self, req: UnlinkProxyMediaRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_name": req.clip_name
        });
        
        self.bridge.call_api("unlink_proxy_media", args).await?;
        Ok(format!("Successfully unlinked proxy media for clip '{}'", req.clip_name))
    }

    pub async fn replace_clip(&self, req: ReplaceClipRequest) -> ResolveResult<String> {
        let args = serde_json::json!({
            "clip_name": req.clip_name,
            "replacement_path": req.replacement_path
        });
        
        self.bridge.call_api("replace_clip", args).await?;
        Ok(format!("Successfully replaced clip '{}' with '{}'", 
                  req.clip_name, req.replacement_path))
    }
}

// ============================================
// TOOL ROUTING FUNCTION
// ============================================

pub async fn handle_tool_call(
    tool_name: &str,
    args: serde_json::Value,
    bridge: Arc<ResolveBridge>,
) -> ResolveResult<String> {
    let project_tools = ProjectTools::new(bridge.clone());
    let timeline_tools = TimelineTools::new(bridge.clone());
    let media_tools = MediaTools::new(bridge.clone());

    match tool_name {
        // ---- Phase 1 & 2 Tools ----
        "create_project" => {
            let req: CreateProjectRequest = serde_json::from_value(args)?;
            project_tools.create_project(req).await
        }
        "open_project" => {
            let req: OpenProjectRequest = serde_json::from_value(args)?;
            project_tools.open_project(req).await
        }
        "switch_page" => {
            let req: SwitchPageRequest = serde_json::from_value(args)?;
            project_tools.switch_page(req).await
        }
        "create_timeline" => {
            let req: CreateTimelineRequest = serde_json::from_value(args)?;
            timeline_tools.create_timeline(req).await
        }
        "import_media" => {
            let req: ImportMediaRequest = serde_json::from_value(args)?;
            media_tools.import_media(req).await
        }
        "add_marker" => {
            let req: AddMarkerRequest = serde_json::from_value(args)?;
            timeline_tools.add_marker(req).await
        }

        // ---- Phase 3 Week 1: New Media Operations ----
        "create_bin" => {
            let req: CreateBinRequest = serde_json::from_value(args)?;
            media_tools.create_bin(req).await
        }
        "auto_sync_audio" => {
            let req: AutoSyncAudioRequest = serde_json::from_value(args)?;
            media_tools.auto_sync_audio(req).await
        }
        "unlink_clips" => {
            let req: UnlinkClipsRequest = serde_json::from_value(args)?;
            media_tools.unlink_clips(req).await
        }
        "relink_clips" => {
            let req: RelinkClipsRequest = serde_json::from_value(args)?;
            media_tools.relink_clips(req).await
        }
        "create_sub_clip" => {
            let req: CreateSubClipRequest = serde_json::from_value(args)?;
            media_tools.create_sub_clip(req).await
        }
        "link_proxy_media" => {
            let req: LinkProxyMediaRequest = serde_json::from_value(args)?;
            media_tools.link_proxy_media(req).await
        }
        "unlink_proxy_media" => {
            let req: UnlinkProxyMediaRequest = serde_json::from_value(args)?;
            media_tools.unlink_proxy_media(req).await
        }
        "replace_clip" => {
            let req: ReplaceClipRequest = serde_json::from_value(args)?;
            media_tools.replace_clip(req).await
        }

        // Timeline Enhancement Tools (Phase 3 Week 2)
        "delete_timeline" => {
            let req: DeleteTimelineRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_timeline", serde_json::json!({
                "name": req.name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_current_timeline" => {
            let req: SetCurrentTimelineRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_current_timeline", serde_json::json!({
                "name": req.name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "create_empty_timeline" => {
            let req: CreateEmptyTimelineRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("create_empty_timeline", serde_json::json!({
                "name": req.name,
                "frame_rate": req.frame_rate,
                "resolution_width": req.resolution_width,
                "resolution_height": req.resolution_height,
                "start_timecode": req.start_timecode,
                "video_tracks": req.video_tracks,
                "audio_tracks": req.audio_tracks
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "add_clip_to_timeline" => {
            let req: AddClipToTimelineRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("add_clip_to_timeline", serde_json::json!({
                "clip_name": req.clip_name,
                "timeline_name": req.timeline_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_timeline_tracks" => {
            let req: GetTimelineTracksRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_tracks", serde_json::json!({
                "timeline_name": req.timeline_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "list_timelines_tool" => {
            let response = bridge.call_api("list_timelines_tool", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- Color Operations Request Types (Phase 3 Week 3) ----
        "apply_lut" => {
            let req: ApplyLutRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("apply_lut", serde_json::json!({
                "lut_path": req.lut_path,
                "node_index": req.node_index
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_color_wheel_param" => {
            let req: SetColorWheelParamRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_color_wheel_param", serde_json::json!({
                "wheel": req.wheel,
                "param": req.param,
                "value": req.value,
                "node_index": req.node_index
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "add_node" => {
            let req: AddNodeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("add_node", serde_json::json!({
                "node_type": req.node_type,
                "label": req.label
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "copy_grade" => {
            let req: CopyGradeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("copy_grade", serde_json::json!({
                "source_clip_name": req.source_clip_name,
                "target_clip_name": req.target_clip_name,
                "mode": req.mode
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "save_color_preset" => {
            let req: SaveColorPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("save_color_preset", serde_json::json!({
                "clip_name": req.clip_name,
                "preset_name": req.preset_name,
                "album_name": req.album_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "apply_color_preset" => {
            let req: ApplyColorPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("apply_color_preset", serde_json::json!({
                "preset_id": req.preset_id,
                "preset_name": req.preset_name,
                "clip_name": req.clip_name,
                "album_name": req.album_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "delete_color_preset" => {
            let req: DeleteColorPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_color_preset", serde_json::json!({
                "preset_id": req.preset_id,
                "preset_name": req.preset_name,
                "album_name": req.album_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_lut" => {
            let req: ExportLutRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_lut", serde_json::json!({
                "clip_name": req.clip_name,
                "export_path": req.export_path,
                "lut_format": req.lut_format,
                "lut_size": req.lut_size
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- Timeline Item Operations Request Types (Phase 4 Week 1) ----
        "set_timeline_item_transform" => {
            let req: SetTimelineItemTransformRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_item_transform", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_name": req.property_name,
                "property_value": req.property_value
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_item_crop" => {
            let req: SetTimelineItemCropRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_item_crop", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "crop_type": req.crop_type,
                "crop_value": req.crop_value
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_item_composite" => {
            let req: SetTimelineItemCompositeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_item_composite", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "composite_mode": req.composite_mode,
                "opacity": req.opacity
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_item_retime" => {
            let req: SetTimelineItemRetimeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_item_retime", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "speed": req.speed,
                "process": req.process
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_item_stabilization" => {
            let req: SetTimelineItemStabilizationRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_item_stabilization", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "enabled": req.enabled,
                "method": req.method,
                "strength": req.strength
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_item_audio" => {
            let req: SetTimelineItemAudioRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_item_audio", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "volume": req.volume,
                "pan": req.pan,
                "eq_enabled": req.eq_enabled
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_timeline_item_properties" => {
            let req: GetTimelineItemPropertiesRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_item_properties", serde_json::json!({
                "timeline_item_id": req.timeline_item_id
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "reset_timeline_item_properties" => {
            let req: ResetTimelineItemPropertiesRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("reset_timeline_item_properties", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_type": req.property_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        _ => Err(crate::error::ResolveError::ToolNotFound {
            name: tool_name.to_string(),
        }),
    }
} 