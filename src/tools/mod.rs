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

// ---- Keyframe Animation Request Types (Phase 4 Week 2) ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddKeyframeRequest {
    #[schemars(description = "The ID of the timeline item to add keyframe to")]
    pub timeline_item_id: String,
    #[schemars(description = "The name of the property to keyframe (e.g., 'Pan', 'ZoomX', 'Opacity')")]
    pub property_name: String,
    #[schemars(description = "Frame position for the keyframe")]
    pub frame: i32,
    #[schemars(description = "Value to set at the keyframe")]
    pub value: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ModifyKeyframeRequest {
    #[schemars(description = "The ID of the timeline item")]
    pub timeline_item_id: String,
    #[schemars(description = "The name of the property with keyframe")]
    pub property_name: String,
    #[schemars(description = "Current frame position of the keyframe to modify")]
    pub frame: i32,
    #[schemars(description = "Optional new value for the keyframe")]
    pub new_value: Option<f64>,
    #[schemars(description = "Optional new frame position for the keyframe")]
    pub new_frame: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteKeyframeRequest {
    #[schemars(description = "The ID of the timeline item")]
    pub timeline_item_id: String,
    #[schemars(description = "The name of the property with keyframe to delete")]
    pub property_name: String,
    #[schemars(description = "Frame position of the keyframe to delete")]
    pub frame: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetKeyframeInterpolationRequest {
    #[schemars(description = "The ID of the timeline item")]
    pub timeline_item_id: String,
    #[schemars(description = "The name of the property with keyframe")]
    pub property_name: String,
    #[schemars(description = "Frame position of the keyframe")]
    pub frame: i32,
    #[schemars(description = "Type of interpolation. Options: 'Linear', 'Bezier', 'Ease-In', 'Ease-Out', 'Hold'")]
    pub interpolation_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EnableKeyframesRequest {
    #[schemars(description = "The ID of the timeline item")]
    pub timeline_item_id: String,
    #[schemars(description = "Keyframe mode to enable. Options: 'All', 'Color', 'Sizing'")]
    #[serde(default = "default_keyframe_mode")]
    pub keyframe_mode: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetKeyframesRequest {
    #[schemars(description = "The ID of the timeline item")]
    pub timeline_item_id: String,
    #[schemars(description = "Optional property name to get keyframes for (returns all if None)")]
    pub property_name: Option<String>,
}

// ---- Render and Delivery Operations (Phase 4 Week 3) ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddToRenderQueueRequest {
    #[schemars(description = "Name of the render preset to use")]
    pub preset_name: String,
    #[schemars(description = "Name of the timeline to render (uses current if None)")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Whether to render only the in/out range instead of entire timeline")]
    #[serde(default)]
    pub use_in_out_range: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StartRenderRequest {
    // No additional parameters needed - starts all queued jobs
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClearRenderQueueRequest {
    // No additional parameters needed - clears all jobs from queue
}

// ---- Project Management Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SaveProjectRequest {
    // No additional parameters needed - saves current project
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloseProjectRequest {
    // No additional parameters needed - closes current project
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetProjectSettingRequest {
    #[schemars(description = "The name of the setting to change")]
    pub setting_name: String,
    #[schemars(description = "The new value for the setting (can be string, integer, float, or boolean)")]
    pub setting_value: serde_json::Value,
}

// ---- Audio Transcription Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TranscribeAudioRequest {
    #[schemars(description = "Name of the clip to transcribe")]
    pub clip_name: String,
    #[schemars(description = "Language code for transcription (default: en-US)")]
    #[serde(default = "default_language")]
    pub language: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClearTranscriptionRequest {
    #[schemars(description = "Name of the clip to clear transcription from")]
    pub clip_name: String,
}

// ---- Phase 4 Week 3: Rendering & Delivery Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetRenderStatusRequest {
    // No additional parameters needed - returns current render status
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportProjectRequest {
    #[schemars(description = "Path to save the exported project")]
    pub export_path: String,
    #[schemars(description = "Whether to include media files in export")]
    #[serde(default)]
    pub include_media: bool,
    #[schemars(description = "Optional custom name for the exported project")]
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateRenderPresetRequest {
    #[schemars(description = "Name for the new render preset")]
    pub preset_name: String,
    #[schemars(description = "Output format (MP4, MOV, MXF, etc.)")]
    pub format: String,
    #[schemars(description = "Video codec (H.264, H.265, ProRes, etc.)")]
    pub codec: String,
    #[schemars(description = "Output width in pixels")]
    pub resolution_width: u32,
    #[schemars(description = "Output height in pixels")]
    pub resolution_height: u32,
    #[schemars(description = "Frame rate")]
    pub frame_rate: f32,
    #[schemars(description = "Quality setting (1-100)")]
    pub quality: u32,
    #[schemars(description = "Audio codec")]
    #[serde(default = "default_audio_codec")]
    pub audio_codec: String,
    #[schemars(description = "Audio bitrate in bps (e.g., 192000 for 192kbps)")]
    #[serde(default = "default_audio_bitrate")]
    pub audio_bitrate: u32,
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

fn default_keyframe_mode() -> String {
    "All".to_string()
}

fn default_language() -> String {
    "en-US".to_string()
}

fn default_audio_codec() -> String {
    "AAC".to_string()
}

fn default_audio_bitrate() -> u32 {
    192000
}

// ---- NEW: Extended Project Management Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteMediaRequest {
    #[schemars(description = "Name of the clip to delete")]
    pub clip_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MoveMediaToBinRequest {
    #[schemars(description = "Name of the clip to move")]
    pub clip_name: String,
    #[schemars(description = "Name of the target bin")]
    pub bin_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportFolderRequest {
    #[schemars(description = "Name of the folder to export")]
    pub folder_name: String,
    #[schemars(description = "Path to save the exported file")]
    pub export_path: String,
    #[schemars(description = "Export format (DRB is default and currently the only supported option)")]
    #[serde(default = "default_export_type")]
    pub export_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TranscribeFolderAudioRequest {
    #[schemars(description = "Name of the folder containing clips to transcribe")]
    pub folder_name: String,
    #[schemars(description = "Language code for transcription (default: en-US)")]
    #[serde(default = "default_language")]
    pub language: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClearFolderTranscriptionRequest {
    #[schemars(description = "Name of the folder to clear transcriptions from")]
    pub folder_name: String,
}

// ---- NEW: Cache and Optimization Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCacheModeRequest {
    #[schemars(description = "Cache mode to set. Options: 'auto', 'on', 'off'")]
    pub mode: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetOptimizedMediaModeRequest {
    #[schemars(description = "Optimized media mode to set. Options: 'auto', 'on', 'off'")]
    pub mode: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetProxyModeRequest {
    #[schemars(description = "Proxy mode to set. Options: 'auto', 'on', 'off'")]
    pub mode: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetProxyQualityRequest {
    #[schemars(description = "Proxy quality to set. Options: 'quarter', 'half', 'threeQuarter', 'full'")]
    pub quality: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCachePathRequest {
    #[schemars(description = "Type of cache path to set. Options: 'local', 'network'")]
    pub path_type: String,
    #[schemars(description = "File system path for the cache")]
    pub path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GenerateOptimizedMediaRequest {
    #[schemars(description = "Optional list of clip names. If None, processes all clips in media pool")]
    pub clip_names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteOptimizedMediaRequest {
    #[schemars(description = "Optional list of clip names. If None, processes all clips in media pool")]
    pub clip_names: Option<Vec<String>>,
}

// ---- NEW: Extended Color Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateColorPresetAlbumRequest {
    #[schemars(description = "Name for the new album")]
    pub album_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteColorPresetAlbumRequest {
    #[schemars(description = "Name of the album to delete")]
    pub album_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportAllPowerGradeLutsRequest {
    #[schemars(description = "Directory to save the exported LUTs")]
    pub export_dir: String,
}

// ---- NEW: Layout and Interface Management ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SaveLayoutPresetRequest {
    #[schemars(description = "Name for the saved preset")]
    pub preset_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoadLayoutPresetRequest {
    #[schemars(description = "Name of the preset to load")]
    pub preset_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportLayoutPresetRequest {
    #[schemars(description = "Name of the preset to export")]
    pub preset_name: String,
    #[schemars(description = "Path to export the preset file to")]
    pub export_path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportLayoutPresetRequest {
    #[schemars(description = "Path to the preset file to import")]
    pub import_path: String,
    #[schemars(description = "Name to save the imported preset as (uses filename if None)")]
    pub preset_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteLayoutPresetRequest {
    #[schemars(description = "Name of the preset to delete")]
    pub preset_name: String,
}

// ---- NEW: Application Control ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QuitAppRequest {
    #[schemars(description = "Whether to force quit even if unsaved changes (potentially dangerous)")]
    #[serde(default)]
    pub force: bool,
    #[schemars(description = "Whether to save the project before quitting")]
    #[serde(default = "default_save_project")]
    pub save_project: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RestartAppRequest {
    #[schemars(description = "Seconds to wait between quit and restart")]
    #[serde(default = "default_wait_seconds")]
    pub wait_seconds: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OpenSettingsRequest {
    // No additional parameters needed
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OpenAppPreferencesRequest {
    // No additional parameters needed
}

// ---- NEW: Cloud Operations ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCloudProjectRequest {
    #[schemars(description = "Name for the new cloud project")]
    pub project_name: String,
    #[schemars(description = "Optional path for the cloud project folder")]
    pub folder_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImportCloudProjectRequest {
    #[schemars(description = "Cloud ID or reference of the project to import")]
    pub cloud_id: String,
    #[schemars(description = "Optional custom name for the imported project (uses original name if None)")]
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RestoreCloudProjectRequest {
    #[schemars(description = "Cloud ID or reference of the project to restore")]
    pub cloud_id: String,
    #[schemars(description = "Optional custom name for the restored project (uses original name if None)")]
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportProjectToCloudRequest {
    #[schemars(description = "Optional name of project to export (uses current project if None)")]
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddUserToCloudProjectRequest {
    #[schemars(description = "Cloud ID of the project")]
    pub cloud_id: String,
    #[schemars(description = "Email of the user to add")]
    pub user_email: String,
    #[schemars(description = "Permission level (viewer, editor, admin)")]
    #[serde(default = "default_permissions")]
    pub permissions: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveUserFromCloudProjectRequest {
    #[schemars(description = "Cloud ID of the project")]
    pub cloud_id: String,
    #[schemars(description = "Email of the user to remove")]
    pub user_email: String,
}

// ---- NEW: Object Inspection ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ObjectHelpRequest {
    #[schemars(description = "Type of object to get help for ('resolve', 'project_manager', 'project', 'media_pool', 'timeline', 'media_storage')")]
    pub object_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InspectCustomObjectRequest {
    #[schemars(description = "Path to the object using dot notation (e.g., 'resolve.GetMediaStorage()')")]
    pub object_path: String,
}

// ---- NEW: Project Properties ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetProjectPropertyRequest {
    #[schemars(description = "Name of the property to set")]
    pub property_name: String,
    #[schemars(description = "Value to set for the property")]
    pub property_value: serde_json::Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineFormatRequest {
    #[schemars(description = "Timeline width in pixels")]
    pub width: i32,
    #[schemars(description = "Timeline height in pixels")]
    pub height: i32,
    #[schemars(description = "Timeline frame rate")]
    pub frame_rate: f64,
    #[schemars(description = "Whether the timeline should use interlaced processing")]
    #[serde(default)]
    pub interlaced: bool,
}

// Helper functions for default values
fn default_export_type() -> String {
    "DRB".to_string()
}

fn default_save_project() -> bool {
    true
}

fn default_wait_seconds() -> i32 {
    5
}

fn default_permissions() -> String {
    "viewer".to_string()
}

// ---- NEW: Timeline Object API ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineNameRequest {
    #[schemars(description = "Timeline name to get")]
    pub timeline_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineNameRequest {
    #[schemars(description = "Timeline name to set")]
    pub timeline_name: String,
    #[schemars(description = "New name for the timeline")]
    pub new_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineFramesRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineTimecodeRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Timecode to set")]
    pub timecode: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineTrackCountRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Track type (video, audio, subtitle)")]
    pub track_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineItemsInTrackRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Track type (video, audio, subtitle)")]
    pub track_type: String,
    #[schemars(description = "Track index")]
    pub track_index: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddTimelineMarkerRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Frame ID for the marker")]
    pub frame_id: f64,
    #[schemars(description = "Marker color")]
    #[serde(default = "default_marker_color")]
    pub color: String,
    #[schemars(description = "Marker name")]
    #[serde(default)]
    pub name: String,
    #[schemars(description = "Marker note")]
    #[serde(default)]
    pub note: String,
    #[schemars(description = "Marker duration")]
    #[serde(default = "default_marker_duration")]
    pub duration: f64,
    #[schemars(description = "Custom data")]
    #[serde(default)]
    pub custom_data: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineMarkersRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTimelineMarkerRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Frame number")]
    pub frame_num: Option<f64>,
    #[schemars(description = "Marker color to delete")]
    pub color: Option<String>,
    #[schemars(description = "Custom data to match")]
    pub custom_data: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DuplicateTimelineRequest {
    #[schemars(description = "Source timeline name")]
    pub source_timeline_name: String,
    #[schemars(description = "New timeline name")]
    pub new_timeline_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCompoundClipRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Timeline item IDs to include")]
    pub timeline_item_ids: Vec<String>,
    #[schemars(description = "Compound clip name")]
    pub clip_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateFusionClipRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Timeline item IDs to include")]
    pub timeline_item_ids: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportTimelineRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Export file name")]
    pub file_name: String,
    #[schemars(description = "Export type (AAF, EDL, XML, FCPXML, DRT, ADL, OTIO)")]
    pub export_type: String,
    #[schemars(description = "Export subtype")]
    pub export_subtype: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InsertGeneratorRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Generator name")]
    pub generator_name: String,
    #[schemars(description = "Generator type (standard, fusion, ofx)")]
    #[serde(default = "default_generator_type")]
    pub generator_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InsertTitleRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Title name")]
    pub title_name: String,
    #[schemars(description = "Title type (standard, fusion)")]
    #[serde(default = "default_title_type")]
    pub title_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GrabStillRequest {
    #[schemars(description = "Timeline name")]
    pub timeline_name: Option<String>,
    #[schemars(description = "Still frame source")]
    pub still_frame_source: Option<String>,
    #[schemars(description = "Grab all stills")]
    #[serde(default)]
    pub grab_all: bool,
}

// ---- NEW: TimelineItem Object API ----
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineItemPropertyRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Property key")]
    pub property_key: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTimelineItemPropertyRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Property key")]
    pub property_key: String,
    #[schemars(description = "Property value")]
    pub property_value: serde_json::Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineItemDetailsRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddTimelineItemMarkerRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Frame ID for the marker")]
    pub frame_id: f64,
    #[schemars(description = "Marker color")]
    #[serde(default = "default_marker_color")]
    pub color: String,
    #[schemars(description = "Marker name")]
    #[serde(default)]
    pub name: String,
    #[schemars(description = "Marker note")]
    #[serde(default)]
    pub note: String,
    #[schemars(description = "Marker duration")]
    #[serde(default = "default_marker_duration")]
    pub duration: f64,
    #[schemars(description = "Custom data")]
    #[serde(default)]
    pub custom_data: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTimelineItemMarkersRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTimelineItemMarkerRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Frame number")]
    pub frame_num: Option<f64>,
    #[schemars(description = "Marker color to delete")]
    pub color: Option<String>,
    #[schemars(description = "Custom data to match")]
    pub custom_data: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimelineItemFlagRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Flag color")]
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimelineItemColorRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Color name")]
    pub color_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FusionCompRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Composition index")]
    pub comp_index: Option<i32>,
    #[schemars(description = "Composition name")]
    pub comp_name: Option<String>,
    #[schemars(description = "File path for import/export")]
    pub file_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VersionRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Version name")]
    pub version_name: String,
    #[schemars(description = "Version type")]
    #[serde(default = "default_version_type")]
    pub version_type: String,
    #[schemars(description = "New version name for rename")]
    pub new_version_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StereoParamsRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Stereo parameters")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NodeLUTRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Node index")]
    pub node_index: i32,
    #[schemars(description = "LUT file path")]
    pub lut_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCDLRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "CDL parameters")]
    pub cdl_map: serde_json::Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TakeRequest {
    #[schemars(description = "Timeline item ID")]
    pub timeline_item_id: String,
    #[schemars(description = "Media pool item for new take")]
    pub media_pool_item: Option<String>,
    #[schemars(description = "Start frame")]
    pub start_frame: Option<f64>,
    #[schemars(description = "End frame")]
    pub end_frame: Option<f64>,
    #[schemars(description = "Take index")]
    pub take_index: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CopyGradesRequest {
    #[schemars(description = "Source timeline item ID")]
    pub source_timeline_item_id: String,
    #[schemars(description = "Target timeline item IDs")]
    pub target_timeline_item_ids: Vec<String>,
}

// Helper functions for defaults
fn default_marker_color() -> String {
    "Blue".to_string()
}

fn default_marker_duration() -> f64 {
    1.0
}

fn default_generator_type() -> String {
    "standard".to_string()
}

fn default_title_type() -> String {
    "standard".to_string()
}

fn default_version_type() -> String {
    "local".to_string()
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

        // ---- Keyframe Animation Request Types (Phase 4 Week 2) ----
        "add_keyframe" => {
            let req: AddKeyframeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("add_keyframe", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_name": req.property_name,
                "frame": req.frame,
                "value": req.value
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "modify_keyframe" => {
            let req: ModifyKeyframeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("modify_keyframe", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_name": req.property_name,
                "frame": req.frame,
                "new_value": req.new_value,
                "new_frame": req.new_frame
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "delete_keyframe" => {
            let req: DeleteKeyframeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_keyframe", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_name": req.property_name,
                "frame": req.frame
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_keyframe_interpolation" => {
            let req: SetKeyframeInterpolationRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_keyframe_interpolation", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_name": req.property_name,
                "frame": req.frame,
                "interpolation_type": req.interpolation_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "enable_keyframes" => {
            let req: EnableKeyframesRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("enable_keyframes", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "keyframe_mode": req.keyframe_mode
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_keyframes" => {
            let req: GetKeyframesRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_keyframes", serde_json::json!({
                "timeline_item_id": req.timeline_item_id,
                "property_name": req.property_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- Render and Delivery Operations (Phase 4 Week 3) ----
        "add_to_render_queue" => {
            let req: AddToRenderQueueRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("add_to_render_queue", serde_json::json!({
                "preset_name": req.preset_name,
                "timeline_name": req.timeline_name,
                "use_in_out_range": req.use_in_out_range
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "start_render" => {
            let response = bridge.call_api("start_render", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "clear_render_queue" => {
            let response = bridge.call_api("clear_render_queue", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- Project Management Operations ----
        "save_project" => {
            let response = bridge.call_api("save_project", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "close_project" => {
            let response = bridge.call_api("close_project", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_project_setting" => {
            let req: SetProjectSettingRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_project_setting", serde_json::json!({
                "setting_name": req.setting_name,
                "setting_value": req.setting_value
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- Audio Transcription Operations ----
        "transcribe_audio" => {
            let req: TranscribeAudioRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("transcribe_audio", serde_json::json!({
                "clip_name": req.clip_name,
                "language": req.language
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "clear_transcription" => {
            let req: ClearTranscriptionRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("clear_transcription", serde_json::json!({
                "clip_name": req.clip_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- Phase 4 Week 3: Rendering & Delivery Operations ----
        "get_render_status" => {
            let response = bridge.call_api("get_render_status", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_project" => {
            let req: ExportProjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_project", serde_json::json!({
                "export_path": req.export_path,
                "include_media": req.include_media,
                "project_name": req.project_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "create_render_preset" => {
            let req: CreateRenderPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("create_render_preset", serde_json::json!({
                "preset_name": req.preset_name,
                "format": req.format,
                "codec": req.codec,
                "resolution_width": req.resolution_width,
                "resolution_height": req.resolution_height,
                "frame_rate": req.frame_rate,
                "quality": req.quality,
                "audio_codec": req.audio_codec,
                "audio_bitrate": req.audio_bitrate
            })).await?;
            // Return full response for create_render_preset to include resolution details
            Ok(response.to_string())
        }

        // ---- NEW: Extended Project Management Operations ----
        "delete_media" => {
            let req: DeleteMediaRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_media", serde_json::json!({
                "clip_name": req.clip_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "move_media_to_bin" => {
            let req: MoveMediaToBinRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("move_media_to_bin", serde_json::json!({
                "clip_name": req.clip_name,
                "bin_name": req.bin_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_folder" => {
            let req: ExportFolderRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_folder", serde_json::json!({
                "folder_name": req.folder_name,
                "export_path": req.export_path,
                "export_type": req.export_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "transcribe_folder_audio" => {
            let req: TranscribeFolderAudioRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("transcribe_folder_audio", serde_json::json!({
                "folder_name": req.folder_name,
                "language": req.language
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "clear_folder_transcription" => {
            let req: ClearFolderTranscriptionRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("clear_folder_transcription", serde_json::json!({
                "folder_name": req.folder_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Cache and Optimization Operations ----
        "set_cache_mode" => {
            let req: SetCacheModeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_cache_mode", serde_json::json!({
                "mode": req.mode
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_optimized_media_mode" => {
            let req: SetOptimizedMediaModeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_optimized_media_mode", serde_json::json!({
                "mode": req.mode
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_proxy_mode" => {
            let req: SetProxyModeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_proxy_mode", serde_json::json!({
                "mode": req.mode
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_proxy_quality" => {
            let req: SetProxyQualityRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_proxy_quality", serde_json::json!({
                "quality": req.quality
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_cache_path" => {
            let req: SetCachePathRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_cache_path", serde_json::json!({
                "path_type": req.path_type,
                "path": req.path
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "generate_optimized_media" => {
            let req: GenerateOptimizedMediaRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("generate_optimized_media", serde_json::json!({
                "clip_names": req.clip_names
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "delete_optimized_media" => {
            let req: DeleteOptimizedMediaRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_optimized_media", serde_json::json!({
                "clip_names": req.clip_names
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Extended Color Operations ----
        "create_color_preset_album" => {
            let req: CreateColorPresetAlbumRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("create_color_preset_album", serde_json::json!({
                "album_name": req.album_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "delete_color_preset_album" => {
            let req: DeleteColorPresetAlbumRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_color_preset_album", serde_json::json!({
                "album_name": req.album_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_all_power_grade_luts" => {
            let req: ExportAllPowerGradeLutsRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_all_power_grade_luts", serde_json::json!({
                "export_dir": req.export_dir
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Layout and Interface Management ----
        "save_layout_preset" => {
            let req: SaveLayoutPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("save_layout_preset", serde_json::json!({
                "preset_name": req.preset_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "load_layout_preset" => {
            let req: LoadLayoutPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("load_layout_preset", serde_json::json!({
                "preset_name": req.preset_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_layout_preset" => {
            let req: ExportLayoutPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_layout_preset", serde_json::json!({
                "preset_name": req.preset_name,
                "export_path": req.export_path
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "import_layout_preset" => {
            let req: ImportLayoutPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("import_layout_preset", serde_json::json!({
                "import_path": req.import_path,
                "preset_name": req.preset_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "delete_layout_preset" => {
            let req: DeleteLayoutPresetRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_layout_preset", serde_json::json!({
                "preset_name": req.preset_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Application Control ----
        "quit_app" => {
            let req: QuitAppRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("quit_app", serde_json::json!({
                "force": req.force,
                "save_project": req.save_project
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "restart_app" => {
            let req: RestartAppRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("restart_app", serde_json::json!({
                "wait_seconds": req.wait_seconds
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "open_settings" => {
            let response = bridge.call_api("open_settings", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "open_app_preferences" => {
            let response = bridge.call_api("open_app_preferences", serde_json::json!({})).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Cloud Operations ----
        "create_cloud_project" => {
            let req: CreateCloudProjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("create_cloud_project", serde_json::json!({
                "project_name": req.project_name,
                "folder_path": req.folder_path
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "import_cloud_project" => {
            let req: ImportCloudProjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("import_cloud_project", serde_json::json!({
                "cloud_id": req.cloud_id,
                "project_name": req.project_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "restore_cloud_project" => {
            let req: RestoreCloudProjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("restore_cloud_project", serde_json::json!({
                "cloud_id": req.cloud_id,
                "project_name": req.project_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_project_to_cloud" => {
            let req: ExportProjectToCloudRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_project_to_cloud", serde_json::json!({
                "project_name": req.project_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "add_user_to_cloud_project" => {
            let req: AddUserToCloudProjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("add_user_to_cloud_project", serde_json::json!({
                "cloud_id": req.cloud_id,
                "user_email": req.user_email,
                "permissions": req.permissions
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "remove_user_from_cloud_project" => {
            let req: RemoveUserFromCloudProjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("remove_user_from_cloud_project", serde_json::json!({
                "cloud_id": req.cloud_id,
                "user_email": req.user_email
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Object Inspection ----
        "object_help" => {
            let req: ObjectHelpRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("object_help", serde_json::json!({
                "object_type": req.object_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "inspect_custom_object" => {
            let req: InspectCustomObjectRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("inspect_custom_object", serde_json::json!({
                "object_path": req.object_path
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Project Properties ----
        "set_project_property" => {
            let req: SetProjectPropertyRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_project_property", serde_json::json!({
                "property_name": req.property_name,
                "property_value": req.property_value
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_format" => {
            let req: SetTimelineFormatRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_format", serde_json::json!({
                "width": req.width,
                "height": req.height,
                "frame_rate": req.frame_rate,
                "interlaced": req.interlaced
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        // ---- NEW: Timeline Object API ----
        "get_timeline_name" => {
            let req: GetTimelineNameRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_name", serde_json::json!({
                "timeline_name": req.timeline_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_name" => {
            let req: SetTimelineNameRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_name", serde_json::json!({
                "timeline_name": req.timeline_name,
                "new_name": req.new_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_timeline_frames" => {
            let req: GetTimelineFramesRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_frames", serde_json::json!({
                "timeline_name": req.timeline_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "set_timeline_timecode" => {
            let req: SetTimelineTimecodeRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("set_timeline_timecode", serde_json::json!({
                "timeline_name": req.timeline_name,
                "timecode": req.timecode
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_timeline_track_count" => {
            let req: GetTimelineTrackCountRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_track_count", serde_json::json!({
                "timeline_name": req.timeline_name,
                "track_type": req.track_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_timeline_items_in_track" => {
            let req: GetTimelineItemsInTrackRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_items_in_track", serde_json::json!({
                "timeline_name": req.timeline_name,
                "track_type": req.track_type,
                "track_index": req.track_index
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "add_timeline_marker" => {
            let req: AddTimelineMarkerRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("add_timeline_marker", serde_json::json!({
                "timeline_name": req.timeline_name,
                "frame_id": req.frame_id,
                "color": req.color,
                "name": req.name,
                "note": req.note,
                "duration": req.duration,
                "custom_data": req.custom_data
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "get_timeline_markers" => {
            let req: GetTimelineMarkersRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("get_timeline_markers", serde_json::json!({
                "timeline_name": req.timeline_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "delete_timeline_marker" => {
            let req: DeleteTimelineMarkerRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("delete_timeline_marker", serde_json::json!({
                "timeline_name": req.timeline_name,
                "frame_num": req.frame_num,
                "color": req.color,
                "custom_data": req.custom_data
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "duplicate_timeline" => {
            let req: DuplicateTimelineRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("duplicate_timeline", serde_json::json!({
                "source_timeline_name": req.source_timeline_name,
                "new_timeline_name": req.new_timeline_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "create_compound_clip" => {
            let req: CreateCompoundClipRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("create_compound_clip", serde_json::json!({
                "timeline_name": req.timeline_name,
                "timeline_item_ids": req.timeline_item_ids,
                "clip_name": req.clip_name
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "create_fusion_clip" => {
            let req: CreateFusionClipRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("create_fusion_clip", serde_json::json!({
                "timeline_name": req.timeline_name,
                "timeline_item_ids": req.timeline_item_ids
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "export_timeline" => {
            let req: ExportTimelineRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("export_timeline", serde_json::json!({
                "timeline_name": req.timeline_name,
                "file_name": req.file_name,
                "export_type": req.export_type,
                "export_subtype": req.export_subtype
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "insert_generator" => {
            let req: InsertGeneratorRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("insert_generator", serde_json::json!({
                "timeline_name": req.timeline_name,
                "generator_name": req.generator_name,
                "generator_type": req.generator_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "insert_title" => {
            let req: InsertTitleRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("insert_title", serde_json::json!({
                "timeline_name": req.timeline_name,
                "title_name": req.title_name,
                "title_type": req.title_type
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }
        "grab_still" => {
            let req: GrabStillRequest = serde_json::from_value(args)?;
            let response = bridge.call_api("grab_still", serde_json::json!({
                "timeline_name": req.timeline_name,
                "still_frame_source": req.still_frame_source,
                "grab_all": req.grab_all
            })).await?;
            Ok(response["result"].as_str().unwrap_or("Success").to_string())
        }

        _ => Err(crate::error::ResolveError::ToolNotFound {
            name: tool_name.to_string(),
        }),
    }
} 