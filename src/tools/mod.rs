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

        _ => Err(crate::error::ResolveError::ToolNotFound {
            name: tool_name.to_string(),
        }),
    }
} 