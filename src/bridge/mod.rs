use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::error::{ResolveError, ResolveResult};
use crate::native::NativeDaVinciResolve;

/// Connection mode for DaVinci Resolve bridge
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionMode {
    /// Simulation mode - uses in-memory state (for testing/development)
    Simulation,
    /// Real mode - attempts to connect to actual DaVinci Resolve instance
    Real,
}

/// Pure Rust implementation of DaVinci Resolve operations
/// This can operate in simulation mode or attempt real connections
#[derive(Debug)]
pub struct ResolveBridge {
    /// Connection mode
    mode: ConnectionMode,
    /// Simulated state for development and testing
    state: Arc<Mutex<ResolveState>>,
    /// Connection status
    connected: Arc<Mutex<bool>>,
    /// Native DaVinci Resolve integration (future feature)
    #[allow(dead_code)]
    native: Arc<Mutex<Option<NativeDaVinciResolve>>>,
}

#[derive(Debug, Default)]
pub struct ResolveState {
    /// Current project name
    current_project: Option<String>,
    /// List of available projects
    projects: Vec<String>,
    /// Current page
    current_page: String,
    /// Timelines in current project
    timelines: HashMap<String, Timeline>,
    /// Current timeline
    current_timeline: Option<String>,
    /// Media pool bins and clips
    media_pool: MediaPool,
    /// Color grading state (Phase 3 Week 3)
    color_state: ColorState,
    /// Operation counter for realistic responses
    operation_count: u64,
    /// Timeline items state (Phase 4 Week 1)
    timeline_items: TimelineItemsState,
    /// Keyframe animation state (Phase 4 Week 2)
    keyframe_state: KeyframeState,
    /// Render and delivery state (Phase 4 Week 3)
    render_state: RenderState,
    /// Response cache for performance optimization
    #[allow(dead_code)]
    response_cache: HashMap<String, (chrono::DateTime<chrono::Utc>, Value)>,
    /// Cache expiry time in seconds
    #[allow(dead_code)]
    cache_ttl_seconds: i64,
}

impl Default for MediaPool {
    fn default() -> Self {
        let mut clips = HashMap::new();
        let mut bins = HashMap::new();

        // Add some default clips for testing
        clips.insert(
            "default_clip".to_string(),
            Clip {
                name: "default_clip".to_string(),
                file_path: "/path/to/default_clip.mp4".to_string(),
                bin: None,
                linked: true,
                proxy_path: None,
            },
        );

        clips.insert(
            "test_video.mp4".to_string(),
            Clip {
                name: "test_video.mp4".to_string(),
                file_path: "/path/to/test_video.mp4".to_string(),
                bin: Some("Test Bin".to_string()),
                linked: true,
                proxy_path: None,
            },
        );

        clips.insert(
            "sample_audio.wav".to_string(),
            Clip {
                name: "sample_audio.wav".to_string(),
                file_path: "/path/to/sample_audio.wav".to_string(),
                bin: Some("Audio Bin".to_string()),
                linked: true,
                proxy_path: None,
            },
        );

        // Add some default bins
        bins.insert(
            "Test Bin".to_string(),
            Bin {
                name: "Test Bin".to_string(),
                clips: vec!["test_video.mp4".to_string()],
            },
        );

        bins.insert(
            "Audio Bin".to_string(),
            Bin {
                name: "Audio Bin".to_string(),
                clips: vec!["sample_audio.wav".to_string()],
            },
        );

        Self { bins, clips }
    }
}

/// Keyframe animation state management (Phase 4 Week 2)
#[derive(Debug, Default)]
struct KeyframeState {
    /// Keyframes by timeline item ID
    timeline_item_keyframes: HashMap<String, TimelineItemKeyframes>,
    /// Global keyframe counter
    keyframe_counter: u64,
}

#[derive(Debug, Clone, Default)]
struct TimelineItemKeyframes {
    /// Timeline item ID
    #[allow(dead_code)]
    timeline_item_id: String,
    /// Keyframes by property name
    property_keyframes: HashMap<String, Vec<Keyframe>>,
    /// Keyframe mode settings
    keyframe_modes: KeyframeModes,
}

#[derive(Debug, Clone)]
struct Keyframe {
    /// Unique keyframe ID
    id: u64,
    /// Frame position
    frame: i32,
    /// Property value at this frame
    value: f64,
    /// Interpolation type to next keyframe
    interpolation: InterpolationType,
    /// Created timestamp
    created_at: String,
}

#[derive(Debug, Clone)]
enum InterpolationType {
    Linear,
    Bezier,
    EaseIn,
    EaseOut,
    Hold,
}

#[derive(Debug, Clone, Default)]
struct KeyframeModes {
    /// All properties keyframe mode enabled
    all_enabled: bool,
    /// Color properties keyframe mode enabled
    color_enabled: bool,
    /// Sizing properties keyframe mode enabled
    sizing_enabled: bool,
}

#[derive(Debug, Clone)]
struct Timeline {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    frame_rate: Option<String>,
    #[allow(dead_code)]
    resolution_width: Option<i32>,
    #[allow(dead_code)]
    resolution_height: Option<i32>,
    markers: Vec<Marker>,
}

#[derive(Debug, Clone)]
struct Marker {
    #[allow(dead_code)]
    frame: Option<i32>,
    #[allow(dead_code)]
    color: String,
    #[allow(dead_code)]
    note: String,
}

#[derive(Debug)]
struct MediaPool {
    bins: HashMap<String, Bin>,
    clips: HashMap<String, Clip>,
}

#[derive(Debug, Clone)]
struct Bin {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    clips: Vec<String>,
}

#[derive(Debug, Clone)]
struct Clip {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    file_path: String,
    #[allow(dead_code)]
    bin: Option<String>,
    #[allow(dead_code)]
    linked: bool,
    #[allow(dead_code)]
    proxy_path: Option<String>,
}

/// Color grading state management (Phase 3 Week 3)
#[derive(Debug, Default)]
struct ColorState {
    /// Current clip being graded
    current_clip: Option<String>,
    /// LUTs available in the system
    available_luts: HashMap<String, LutInfo>,
    /// Color presets
    color_presets: HashMap<String, ColorPreset>,
    /// Clip grades (per clip)
    clip_grades: HashMap<String, ClipGrade>,
    /// Current node index for grading
    current_node_index: i32,
}

/// Timeline item state management (Phase 4 Week 1)
#[derive(Debug, Default)]
struct TimelineItemsState {
    /// Timeline items by ID
    items: HashMap<String, TimelineItemState>,
    /// Current item counter for ID generation
    item_counter: u64,
}

#[derive(Debug, Clone, Default)]
struct TimelineItemState {
    /// Unique timeline item ID
    #[allow(dead_code)]
    id: String,
    /// Timeline name this item belongs to
    timeline_name: String,
    /// Clip name this item references
    clip_name: String,
    /// Transform properties
    transform: TransformProperties,
    /// Crop settings
    crop: CropProperties,
    /// Composite settings
    composite: CompositeProperties,
    /// Retiming settings
    retime: RetimeProperties,
    /// Stabilization settings
    stabilization: StabilizationProperties,
    /// Audio properties
    audio: AudioProperties,
}

#[derive(Debug, Clone, Default)]
struct TransformProperties {
    pan: f64,
    tilt: f64,
    zoom_x: f64,
    zoom_y: f64,
    rotation: f64,
    anchor_point_x: f64,
    anchor_point_y: f64,
    pitch: f64,
    yaw: f64,
}

#[derive(Debug, Clone, Default)]
struct CropProperties {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

#[derive(Debug, Clone, Default)]
struct CompositeProperties {
    mode: String, // "Normal", "Add", "Multiply", etc.
    opacity: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, Default)]
struct RetimeProperties {
    speed: f64,      // Speed factor
    process: String, // "NearestFrame", "FrameBlend", "OpticalFlow"
}

#[derive(Debug, Clone, Default)]
struct StabilizationProperties {
    enabled: bool,
    method: String, // "Perspective", "Similarity", "Translation"
    strength: f64,  // 0.0 to 1.0
}

#[derive(Debug, Clone, Default)]
struct AudioProperties {
    volume: f64, // Volume level (usually 0.0 to 2.0, where 1.0 is unity gain)
    pan: f64,    // -1.0 to 1.0
    eq_enabled: bool,
}

#[derive(Debug, Clone)]
struct LutInfo {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    path: String,
    #[allow(dead_code)]
    format: String, // "Cube", "Davinci", "3dl", "Panasonic"
    #[allow(dead_code)]
    size: String, // "17Point", "33Point", "65Point"
}

#[derive(Debug, Clone)]
struct ColorPreset {
    name: String,
    #[allow(dead_code)]
    album: String,
    #[allow(dead_code)]
    created_at: String,
    grade_data: ClipGrade,
}

#[derive(Debug, Clone, Default)]
struct ClipGrade {
    /// Color wheel parameters
    lift: ColorWheelParams,
    gamma: ColorWheelParams,
    gain: ColorWheelParams,
    offset: ColorWheelParams,
    /// Applied LUTs
    applied_luts: Vec<String>,
    /// Number of nodes
    node_count: i32,
    /// Node labels
    node_labels: HashMap<i32, String>,
}

#[derive(Debug, Clone, Default)]
struct ColorWheelParams {
    red: f64,
    green: f64,
    blue: f64,
    master: f64,
}

/// Render and delivery state management (Phase 4 Week 3)
#[derive(Debug, Default)]
struct RenderState {
    /// Active render queue
    render_queue: Vec<RenderJob>,
    /// Active render progress tracking
    active_renders: HashMap<String, RenderProgress>,
    /// Available render presets
    render_presets: HashMap<String, RenderPreset>,
    /// Render job history
    render_history: Vec<RenderResult>,
    /// Global render job counter
    job_counter: u64,
}

#[derive(Debug, Clone)]
struct RenderJob {
    /// Unique job ID
    id: String,
    /// Timeline name to render
    timeline_name: String,
    /// Render preset name
    preset_name: String,
    /// Output file path
    output_path: String,
    /// Use in/out range
    use_in_out_range: bool,
    /// Job creation timestamp
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
    /// Current job status
    status: RenderJobStatus,
}

#[derive(Debug, Clone)]
enum RenderJobStatus {
    Queued,
    Rendering,
    #[allow(dead_code)]
    Completed,
    #[allow(dead_code)]
    Failed,
    #[allow(dead_code)]
    Cancelled,
}

#[derive(Debug, Clone)]
struct RenderProgress {
    /// Job ID being tracked
    job_id: String,
    /// Progress percentage (0.0 to 100.0)
    progress_percent: f32,
    /// Estimated time remaining
    estimated_time_remaining: Option<std::time::Duration>,
    /// Current frame being rendered
    current_frame: u32,
    /// Total frames to render
    total_frames: u32,
    /// Current status message
    status_message: String,
    /// Last update timestamp
    #[allow(dead_code)]
    last_update: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
struct RenderPreset {
    /// Preset name
    #[allow(dead_code)]
    name: String,
    /// Output format (MP4, MOV, MXF, etc.)
    #[allow(dead_code)]
    format: String,
    /// Video codec (H.264, H.265, ProRes, etc.)
    #[allow(dead_code)]
    codec: String,
    /// Output resolution
    #[allow(dead_code)]
    resolution: (u32, u32),
    /// Frame rate
    #[allow(dead_code)]
    frame_rate: f32,
    /// Quality setting
    #[allow(dead_code)]
    quality: RenderQuality,
    /// Audio codec
    #[allow(dead_code)]
    audio_codec: String,
    /// Audio bitrate (kbps)
    #[allow(dead_code)]
    audio_bitrate: u32,
    /// Preset creation timestamp
    #[allow(dead_code)]
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
enum RenderQuality {
    #[allow(dead_code)]
    Low,
    #[allow(dead_code)]
    Medium,
    High,
    #[allow(dead_code)]
    Custom(u32), // Custom bitrate in kbps
}

#[derive(Debug, Clone)]
struct RenderResult {
    /// Job ID
    #[allow(dead_code)]
    job_id: String,
    /// Timeline name
    #[allow(dead_code)]
    timeline_name: String,
    /// Preset used
    #[allow(dead_code)]
    preset_name: String,
    /// Output path
    #[allow(dead_code)]
    output_path: String,
    /// Render duration
    #[allow(dead_code)]
    render_duration: std::time::Duration,
    /// Final status
    #[allow(dead_code)]
    status: RenderJobStatus,
    /// Completion timestamp
    #[allow(dead_code)]
    completed_at: chrono::DateTime<chrono::Utc>,
    /// Error message (if failed)
    #[allow(dead_code)]
    error_message: Option<String>,
}

impl ResolveBridge {
    /// Create a new bridge instance
    pub fn new(mode: ConnectionMode) -> Self {
        let mut state = ResolveState::default();
        state.current_page = "media".to_string();

        // Add some default projects for testing
        state.projects = vec![
            "Sample Project".to_string(),
            "Test Timeline".to_string(),
            "Demo Workflow".to_string(),
        ];

        // Initialize color state with sample LUTs and presets (Phase 3 Week 3)
        state.color_state.available_luts.insert(
            "Rec709_to_sRGB".to_string(),
            LutInfo {
                name: "Rec709 to sRGB".to_string(),
                path: "/usr/share/davinci/luts/rec709_to_srgb.cube".to_string(),
                format: "Cube".to_string(),
                size: "33Point".to_string(),
            },
        );
        state.color_state.available_luts.insert(
            "Cinematic_Look".to_string(),
            LutInfo {
                name: "Cinematic Look".to_string(),
                path: "/usr/share/davinci/luts/cinematic.cube".to_string(),
                format: "Cube".to_string(),
                size: "33Point".to_string(),
            },
        );

        Self {
            mode,
            state: Arc::new(Mutex::new(state)),
            connected: Arc::new(Mutex::new(false)),
            native: Arc::new(Mutex::new(None)),
        }
    }

    /// Initialize the bridge with real or simulation connection
    pub async fn initialize(&self) -> ResolveResult<()> {
        match self.mode {
            ConnectionMode::Simulation => {
                tracing::info!("Initialized DaVinci Resolve bridge in SIMULATION mode");
                *self.connected.lock().await = true;
                Ok(())
            }
            ConnectionMode::Real => {
                tracing::info!("Attempting to connect to real DaVinci Resolve instance...");

                // Test Python API connection
                match self.test_python_api_connection().await {
                    Ok(()) => {
                        tracing::info!("✅ Python API connection established successfully");
                        *self.connected.lock().await = true;
                        Ok(())
                    }
                    Err(e) => {
                        tracing::error!("❌ Python API connection failed: {}", e);
                        *self.connected.lock().await = false;
                        Err(e)
                    }
                }
            }
        }
    }

    /// Check if bridge is connected
    pub async fn is_connected(&self) -> bool {
        *self.connected.lock().await
    }

    /// Get connection mode
    pub fn get_mode(&self) -> ConnectionMode {
        self.mode.clone()
    }

    /// Call a DaVinci Resolve API method
    pub async fn call_api(&self, method: &str, args: Value) -> ResolveResult<Value> {
        tracing::debug!(
            "API call: {} with args: {} (mode: {:?})",
            method,
            args,
            self.mode
        );

        // Check if we should use real DaVinci Resolve API
        match self.mode {
            ConnectionMode::Real => {
                // Try to use real DaVinci Resolve API first
                match self.call_real_api(method, &args).await {
                    Ok(result) => {
                        tracing::info!("Real API call successful for {}", method);
                        return Ok(result);
                    }
                    Err(e) => {
                        // Fall back to simulation if real API fails
                        tracing::warn!(
                            "Real API call failed for {} ({}), falling back to simulation",
                            method,
                            e
                        );
                    }
                }
            }
            ConnectionMode::Simulation => {
                // Use simulation mode directly
                tracing::debug!("Using simulation mode for {}", method);
            }
        }

        // Simulation mode logic
        let mut state = self.state.lock().await;
        state.operation_count += 1;

        match method {
            // Project operations
            "create_project" => self.create_project(&mut state, args).await,
            "open_project" => self.open_project(&mut state, args).await,
            "switch_page" => self.switch_page(&mut state, args).await,

            // Timeline operations
            "create_timeline" => self.create_timeline(&mut state, args).await,
            "add_marker" => self.add_marker(&mut state, args).await,

            // Media operations
            "import_media" => self.import_media(&mut state, args).await,
            "create_bin" => self.create_bin(&mut state, args).await,
            "auto_sync_audio" => self.auto_sync_audio(&mut state, args).await,
            "unlink_clips" => self.unlink_clips(&mut state, args).await,
            "relink_clips" => self.relink_clips(&mut state, args).await,
            "create_sub_clip" => self.create_sub_clip(&mut state, args).await,
            "link_proxy_media" => self.link_proxy_media(&mut state, args).await,
            "unlink_proxy_media" => self.unlink_proxy_media(&mut state, args).await,
            "replace_clip" => self.replace_clip(&mut state, args).await,

            // Timeline Enhancement operations (Phase 3 Week 2)
            "delete_timeline" => self.delete_timeline(&mut state, args).await,
            "set_current_timeline" => self.set_current_timeline(&mut state, args).await,
            "create_empty_timeline" => self.create_empty_timeline(&mut state, args).await,
            "add_clip_to_timeline" => self.add_clip_to_timeline(&mut state, args).await,
            "list_timelines_tool" => self.list_timelines_tool(&mut state, args).await,
            "get_timeline_tracks" => self.get_timeline_tracks(&mut state, args).await,

            // Color Operations (Phase 3 Week 3)
            "apply_lut" => self.apply_lut(&mut state, args).await,
            "set_color_wheel_param" => self.set_color_wheel_param(&mut state, args).await,
            "add_node" => self.add_node(&mut state, args).await,
            "copy_grade" => self.copy_grade(&mut state, args).await,
            "save_color_preset" => self.save_color_preset(&mut state, args).await,
            "apply_color_preset" => self.apply_color_preset(&mut state, args).await,
            "delete_color_preset" => self.delete_color_preset(&mut state, args).await,
            "export_lut" => self.export_lut(&mut state, args).await,

            // Timeline Item Operations (Phase 4 Week 1)
            "set_timeline_item_transform" => {
                self.set_timeline_item_transform(&mut state, args).await
            }
            "set_timeline_item_crop" => self.set_timeline_item_crop(&mut state, args).await,
            "set_timeline_item_composite" => {
                self.set_timeline_item_composite(&mut state, args).await
            }
            "set_timeline_item_retime" => self.set_timeline_item_retime(&mut state, args).await,
            "set_timeline_item_stabilization" => {
                self.set_timeline_item_stabilization(&mut state, args).await
            }
            "set_timeline_item_audio" => self.set_timeline_item_audio(&mut state, args).await,
            "get_timeline_item_properties" => {
                self.get_timeline_item_properties(&mut state, args).await
            }
            "reset_timeline_item_properties" => {
                self.reset_timeline_item_properties(&mut state, args).await
            }

            // Keyframe Animation Operations (Phase 4 Week 2)
            "add_keyframe" => self.add_keyframe(&mut state, args).await,
            "modify_keyframe" => self.modify_keyframe(&mut state, args).await,
            "delete_keyframe" => self.delete_keyframe(&mut state, args).await,
            "set_keyframe_interpolation" => self.set_keyframe_interpolation(&mut state, args).await,
            "enable_keyframes" => self.enable_keyframes(&mut state, args).await,
            "get_keyframes" => self.get_keyframes(&mut state, args).await,

            // Render & Delivery Operations (Phase 4 Week 3)
            "add_to_render_queue" => self.add_to_render_queue(&mut state, args).await,
            "start_render" => self.start_render(&mut state, args).await,
            "clear_render_queue" => self.clear_render_queue(&mut state, args).await,
            "get_render_status" => self.get_render_status(&mut state, args).await,
            "export_project" => self.export_project(&mut state, args).await,
            "create_render_preset" => self.create_render_preset(&mut state, args).await,

            // Project Management Operations
            "save_project" => self.save_project(&mut state, args).await,
            "close_project" => self.close_project(&mut state, args).await,
            "set_project_setting" => self.set_project_setting(&mut state, args).await,

            // Audio Transcription Operations
            "transcribe_audio" => self.transcribe_audio(&mut state, args).await,
            "clear_transcription" => self.clear_transcription(&mut state, args).await,

            // Extended Project Management Operations
            "delete_media" => self.delete_media(&mut state, args).await,
            "move_media_to_bin" => self.move_media_to_bin(&mut state, args).await,
            "export_folder" => self.export_folder(&mut state, args).await,
            "transcribe_folder_audio" => self.transcribe_folder_audio(&mut state, args).await,
            "clear_folder_transcription" => self.clear_folder_transcription(&mut state, args).await,

            // Cache and Optimization Operations
            "set_cache_mode" => self.set_cache_mode(&mut state, args).await,
            "set_optimized_media_mode" => self.set_optimized_media_mode(&mut state, args).await,
            "set_proxy_mode" => self.set_proxy_mode(&mut state, args).await,
            "set_proxy_quality" => self.set_proxy_quality(&mut state, args).await,
            "set_cache_path" => self.set_cache_path(&mut state, args).await,
            "generate_optimized_media" => self.generate_optimized_media(&mut state, args).await,
            "delete_optimized_media" => self.delete_optimized_media(&mut state, args).await,

            // Extended Color Operations
            "create_color_preset_album" => self.create_color_preset_album(&mut state, args).await,
            "delete_color_preset_album" => self.delete_color_preset_album(&mut state, args).await,
            "export_all_power_grade_luts" => {
                self.export_all_power_grade_luts(&mut state, args).await
            }

            // Layout and Interface Management
            "save_layout_preset" => self.save_layout_preset(&mut state, args).await,
            "load_layout_preset" => self.load_layout_preset(&mut state, args).await,
            "export_layout_preset" => self.export_layout_preset(&mut state, args).await,
            "import_layout_preset" => self.import_layout_preset(&mut state, args).await,
            "delete_layout_preset" => self.delete_layout_preset(&mut state, args).await,

            // Application Control
            "quit_app" => self.quit_app(&mut state, args).await,
            "restart_app" => self.restart_app(&mut state, args).await,
            "open_settings" => self.open_settings(&mut state, args).await,
            "open_app_preferences" => self.open_app_preferences(&mut state, args).await,

            // Cloud Operations
            "create_cloud_project" => self.create_cloud_project(&mut state, args).await,
            "import_cloud_project" => self.import_cloud_project(&mut state, args).await,
            "restore_cloud_project" => self.restore_cloud_project(&mut state, args).await,
            "export_project_to_cloud" => self.export_project_to_cloud(&mut state, args).await,
            "add_user_to_cloud_project" => self.add_user_to_cloud_project(&mut state, args).await,
            "remove_user_from_cloud_project" => {
                self.remove_user_from_cloud_project(&mut state, args).await
            }

            // Object Inspection
            "object_help" => self.object_help(&mut state, args).await,
            "inspect_custom_object" => self.inspect_custom_object(&mut state, args).await,

            // Project Properties
            "set_project_property" => self.set_project_property(&mut state, args).await,
            "set_timeline_format" => self.set_timeline_format(&mut state, args).await,

            // ---- NEW: Timeline Object API ----
            "get_timeline_name" => self.get_timeline_name(&mut state, args).await,
            "set_timeline_name" => self.set_timeline_name(&mut state, args).await,
            "get_timeline_frames" => self.get_timeline_frames(&mut state, args).await,
            "set_timeline_timecode" => self.set_timeline_timecode(&mut state, args).await,
            "get_timeline_track_count" => self.get_timeline_track_count(&mut state, args).await,
            "get_timeline_items_in_track" => {
                self.get_timeline_items_in_track(&mut state, args).await
            }
            "add_timeline_marker" => self.add_timeline_marker(&mut state, args).await,
            "get_timeline_markers" => self.get_timeline_markers(&mut state, args).await,
            "delete_timeline_marker" => self.delete_timeline_marker(&mut state, args).await,
            "duplicate_timeline" => self.duplicate_timeline(&mut state, args).await,
            "create_compound_clip" => self.create_compound_clip(&mut state, args).await,
            "create_fusion_clip" => self.create_fusion_clip(&mut state, args).await,
            "export_timeline" => self.export_timeline(&mut state, args).await,
            "insert_generator" => self.insert_generator(&mut state, args).await,
            "insert_title" => self.insert_title(&mut state, args).await,
            "grab_still" => self.grab_still(&mut state, args).await,

            // ---- NEW: TimelineItem Object API ----
            "get_timeline_item_property" => self.get_timeline_item_property(&mut state, args).await,
            "set_timeline_item_property" => self.set_timeline_item_property(&mut state, args).await,
            "get_timeline_item_details" => self.get_timeline_item_details(&mut state, args).await,
            "add_timeline_item_marker" => self.add_timeline_item_marker(&mut state, args).await,
            "get_timeline_item_markers" => self.get_timeline_item_markers(&mut state, args).await,
            "delete_timeline_item_marker" => {
                self.delete_timeline_item_marker(&mut state, args).await
            }
            "timeline_item_flag" => self.timeline_item_flag(&mut state, args).await,
            "timeline_item_color" => self.timeline_item_color(&mut state, args).await,
            "fusion_comp" => self.fusion_comp(&mut state, args).await,
            "version" => self.version(&mut state, args).await,
            "stereo_params" => self.stereo_params(&mut state, args).await,
            "node_lut" => self.node_lut(&mut state, args).await,
            "set_cdl" => self.set_cdl(&mut state, args).await,
            "take" => self.take(&mut state, args).await,
            "copy_grades" => self.copy_grades(&mut state, args).await,

            // ---- NEW: MediaPoolItem Object API ----
            "get_media_pool_item_list" => self.get_media_pool_item_list(&mut state, args).await,
            "get_media_pool_item_name" => self.get_media_pool_item_name(&mut state, args).await,
            "set_media_pool_item_name" => self.set_media_pool_item_name(&mut state, args).await,
            "get_media_pool_item_property" => {
                self.get_media_pool_item_property(&mut state, args).await
            }
            "set_media_pool_item_property" => {
                self.set_media_pool_item_property(&mut state, args).await
            }
            "get_media_pool_item_metadata" => {
                self.get_media_pool_item_metadata(&mut state, args).await
            }
            "set_media_pool_item_metadata" => {
                self.set_media_pool_item_metadata(&mut state, args).await
            }
            "add_media_pool_item_marker" => self.add_media_pool_item_marker(&mut state, args).await,
            "get_media_pool_item_markers" => {
                self.get_media_pool_item_markers(&mut state, args).await
            }
            "add_media_pool_item_flag" => self.add_media_pool_item_flag(&mut state, args).await,
            "get_media_pool_item_flag_list" => {
                self.get_media_pool_item_flag_list(&mut state, args).await
            }
            "get_media_pool_item_clip_color" => {
                self.get_media_pool_item_clip_color(&mut state, args).await
            }
            "set_media_pool_item_clip_color" => {
                self.set_media_pool_item_clip_color(&mut state, args).await
            }
            "link_media_pool_item_proxy_media" => {
                self.link_media_pool_item_proxy_media(&mut state, args)
                    .await
            }
            "unlink_media_pool_item_proxy_media" => {
                self.unlink_media_pool_item_proxy_media(&mut state, args)
                    .await
            }
            "transcribe_media_pool_item_audio" => {
                self.transcribe_media_pool_item_audio(&mut state, args)
                    .await
            }
            "clear_media_pool_item_transcription" => {
                self.clear_media_pool_item_transcription(&mut state, args)
                    .await
            }

            // ---- NEW: Missing API Methods ----
            "get_fusion_tool_list" => self.get_fusion_tool_list(&mut state, args).await,
            "get_audio_track_count" => self.get_audio_track_count(&mut state, args).await,
            "get_project_timeline_count" => self.get_project_timeline_count(&mut state, args).await,
            "get_gallery_still_albums" => self.get_gallery_still_albums(&mut state, args).await,
            "get_media_pool_root_folder" => self.get_media_pool_root_folder(&mut state, args).await,
            "add_fusion_tool" => self.add_fusion_tool(&mut state, args).await,
            "get_audio_track_name" => self.get_audio_track_name(&mut state, args).await,
            "set_audio_track_name" => self.set_audio_track_name(&mut state, args).await,
            "add_gallery_still_album" => self.add_gallery_still_album(&mut state, args).await,
            "add_media_pool_sub_folder" => self.add_media_pool_sub_folder(&mut state, args).await,
            "append_to_timeline" => self.append_to_timeline(&mut state, args).await,
            "get_project_timeline_by_index" => {
                self.get_project_timeline_by_index(&mut state, args).await
            }
            "get_project_current_timeline" => {
                self.get_project_current_timeline(&mut state, args).await
            }
            "set_project_current_timeline" => {
                self.set_project_current_timeline(&mut state, args).await
            }
            "get_project_name" => self.get_project_name(&mut state, args).await,
            "set_project_name" => self.set_project_name(&mut state, args).await,
            "get_project_unique_id" => self.get_project_unique_id(&mut state, args).await,
            "get_project_render_job_list" => {
                self.get_project_render_job_list(&mut state, args).await
            }
            "start_project_rendering" => self.start_project_rendering(&mut state, args).await,
            "stop_project_rendering" => self.stop_project_rendering(&mut state, args).await,
            "is_project_rendering_in_progress" => {
                self.is_project_rendering_in_progress(&mut state, args)
                    .await
            }
            "get_project_preset_list" => self.get_project_preset_list(&mut state, args).await,
            "load_project_render_preset" => self.load_project_render_preset(&mut state, args).await,
            "save_as_new_project_render_preset" => {
                self.save_as_new_project_render_preset(&mut state, args)
                    .await
            }
            "get_current_project_render_format_and_codec" => {
                self.get_current_project_render_format_and_codec(&mut state, args)
                    .await
            }
            "set_current_project_render_format_and_codec" => {
                self.set_current_project_render_format_and_codec(&mut state, args)
                    .await
            }
            "get_current_project_render_mode" => {
                self.get_current_project_render_mode(&mut state, args).await
            }
            "set_current_project_render_mode" => {
                self.set_current_project_render_mode(&mut state, args).await
            }
            "get_project_color_groups_list" => {
                self.get_project_color_groups_list(&mut state, args).await
            }
            "add_project_color_group" => self.add_project_color_group(&mut state, args).await,
            "delete_project_color_group" => self.delete_project_color_group(&mut state, args).await,

            _ => Err(ResolveError::not_supported(format!(
                "API method: {}",
                method
            ))),
        }
    }

    /// Call real DaVinci Resolve API using Python integration
    async fn call_real_api(&self, method: &str, args: &Value) -> ResolveResult<Value> {
        use std::process::Command;

        tracing::debug!(
            "Calling real DaVinci Resolve API: {} with args: {}",
            method,
            args
        );

        // Create Python script for the specific API call
        let python_script = match method {
            "switch_page" => {
                let page = args["page"].as_str().unwrap_or("edit");
                format!(r#"
import sys
import json
sys.path.append("/opt/resolve/Developer/Scripting/Modules")

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if not resolve:
        print(json.dumps({{"error": "Cannot connect to DaVinci Resolve"}}))
        sys.exit(1)
    
    result = resolve.OpenPage("{}")
    print(json.dumps({{"success": True, "result": "Switched to {} page", "returned": result}}))
except Exception as e:
    print(json.dumps({{"error": str(e)}}))
    sys.exit(1)
"#, page, page)
            },
            "create_empty_timeline" => {
                let name = args["name"].as_str().unwrap_or("New Timeline");
                // Add timestamp to make timeline name unique
                let unique_name = format!("{} {}", name, chrono::Utc::now().timestamp());
                format!(r#"
import sys
import json
import time
sys.path.append("/opt/resolve/Developer/Scripting/Modules")

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if not resolve:
        print(json.dumps({{"error": "Cannot connect to DaVinci Resolve"}}))
        sys.exit(1)
    
    project_manager = resolve.GetProjectManager()
    project = project_manager.GetCurrentProject()
    if not project:
        print(json.dumps({{"error": "No project open"}}))
        sys.exit(1)
    
    media_pool = project.GetMediaPool()
    timeline = media_pool.CreateEmptyTimeline("{}")
    
    if timeline:
        timeline_name = timeline.GetName()
        print(json.dumps({{"success": True, "result": "Created timeline '{}'", "timeline_name": timeline_name}}))
    else:
        print(json.dumps({{"error": "Failed to create timeline"}}))
        sys.exit(1)
except Exception as e:
    print(json.dumps({{"error": str(e)}}))
    sys.exit(1)
"#, unique_name, unique_name)
            },
            "add_marker" => {
                let frame = args["frame"].as_i64().unwrap_or(0);
                let color = args["color"].as_str().unwrap_or("Blue");
                let note = args["note"].as_str().unwrap_or("");
                format!(r#"
import sys
import json
sys.path.append("/opt/resolve/Developer/Scripting/Modules")

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if not resolve:
        print(json.dumps({{"error": "Cannot connect to DaVinci Resolve"}}))
        sys.exit(1)
    
    project_manager = resolve.GetProjectManager()
    project = project_manager.GetCurrentProject()
    if not project:
        print(json.dumps({{"error": "No project open"}}))
        sys.exit(1)
    
    timeline = project.GetCurrentTimeline()
    if not timeline:
        print(json.dumps({{"error": "No timeline selected"}}))
        sys.exit(1)
    
    result = timeline.AddMarker({}, "{}", "{}", "{}", 1)
    if result:
        print(json.dumps({{"success": True, "result": "Added {} marker at frame {}"}}))
    else:
        print(json.dumps({{"error": "Failed to add marker"}}))
        sys.exit(1)
except Exception as e:
    print(json.dumps({{"error": str(e)}}))
    sys.exit(1)
"#, frame, color, note, note, color, frame)
            },
            "list_timelines_tool" => {
                r#"
import sys
import json
sys.path.append("/opt/resolve/Developer/Scripting/Modules")

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if not resolve:
        print(json.dumps({"error": "Cannot connect to DaVinci Resolve"}))
        sys.exit(1)
    
    project_manager = resolve.GetProjectManager()
    project = project_manager.GetCurrentProject()
    if not project:
        print(json.dumps({"error": "No project open"}))
        sys.exit(1)
    
    timeline_count = project.GetTimelineCount()
    timelines = []
    
    for i in range(1, timeline_count + 1):
        timeline = project.GetTimelineByIndex(i)
        if timeline:
            timelines.append({
                "name": timeline.GetName(),
                "frame_rate": timeline.GetSetting("timelineFrameRate"),
                "resolution": f"{timeline.GetSetting('timelineResolutionWidth')}x{timeline.GetSetting('timelineResolutionHeight')}"
            })
    
    print(json.dumps({"success": True, "timelines": timelines, "count": len(timelines)}))
except Exception as e:
    print(json.dumps({"error": str(e)}))
    sys.exit(1)
"#.to_string()
            },
            _ => {
                return Err(ResolveError::not_supported(format!("Real API method: {}", method)));
            }
        };

        // Execute Python script
        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_script)
            .output()
            .map_err(|e| {
                ResolveError::internal(&format!("Failed to execute Python script: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ResolveError::api_call(
                method,
                format!("Python script failed: {}", stderr),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json_result: serde_json::Value = serde_json::from_str(&stdout).map_err(|e| {
            ResolveError::internal(&format!("Failed to parse Python response: {}", e))
        })?;

        if let Some(_error) = json_result.get("error") {
            return Err(ResolveError::api_call(
                method,
                _error.as_str().unwrap_or("Unknown error").to_string(),
            ));
        }

        if json_result
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            Ok(json_result)
        } else {
            Err(ResolveError::api_call(
                method,
                "API call did not return success".to_string(),
            ))
        }
    }

    /// Test Python API connection to DaVinci Resolve
    async fn test_python_api_connection(&self) -> ResolveResult<()> {
        use std::process::Command;

        tracing::debug!("Testing Python API connection to DaVinci Resolve...");

        let python_script = r#"
import sys
import json
sys.path.append("/opt/resolve/Developer/Scripting/Modules")

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if not resolve:
        print(json.dumps({"error": "Cannot connect to DaVinci Resolve"}))
        sys.exit(1)
    
    project_manager = resolve.GetProjectManager()
    if not project_manager:
        print(json.dumps({"error": "Cannot get project manager"}))
        sys.exit(1)
    
    print(json.dumps({"success": True, "message": "Connection successful"}))
except ImportError as e:
    print(json.dumps({"error": f"Cannot import DaVinciResolveScript: {e}"}))
    sys.exit(1)
except Exception as e:
    print(json.dumps({"error": str(e)}))
    sys.exit(1)
"#;

        let output = Command::new("python3")
            .arg("-c")
            .arg(python_script)
            .output()
            .map_err(|e| {
                ResolveError::internal(&format!("Failed to execute Python test script: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ResolveError::internal(&format!(
                "Python test script failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json_result: serde_json::Value = serde_json::from_str(&stdout).map_err(|e| {
            ResolveError::internal(&format!("Failed to parse Python test response: {}", e))
        })?;

        if let Some(_error) = json_result.get("error") {
            return Err(ResolveError::NotRunning);
        }

        if json_result
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            tracing::info!("🐍 Python API connection test successful");
            Ok(())
        } else {
            Err(ResolveError::NotRunning)
        }
    }

    async fn create_project(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        if state.projects.contains(&name.to_string()) {
            return Err(ResolveError::invalid_parameter(
                "name",
                "project already exists",
            ));
        }

        state.projects.push(name.to_string());
        state.current_project = Some(name.to_string());
        state.timelines.clear();
        state.media_pool = MediaPool::default();

        Ok(serde_json::json!({
            "result": format!("Created project '{}'", name),
            "project_id": Uuid::new_v4().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    async fn open_project(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        if !state.projects.contains(&name.to_string()) {
            return Err(ResolveError::ProjectNotFound {
                name: name.to_string(),
            });
        }

        state.current_project = Some(name.to_string());

        // Simulate loading existing timelines and media
        if !state.timelines.contains_key(name) {
            state.timelines.insert(
                name.to_string(),
                Timeline {
                    name: format!("{} Timeline", name),
                    frame_rate: Some("24".to_string()),
                    resolution_width: Some(1920),
                    resolution_height: Some(1080),
                    markers: vec![],
                },
            );
        }

        Ok(serde_json::json!({
            "result": format!("Opened project '{}'", name),
            "timelines": state.timelines.len(),
            "media_clips": state.media_pool.clips.len()
        }))
    }

    async fn switch_page(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let page = args["page"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("page", "required string"))?;

        let valid_pages = vec![
            "media",
            "cut",
            "edit",
            "fusion",
            "color",
            "fairlight",
            "deliver",
        ];
        if !valid_pages.contains(&page) {
            return Err(ResolveError::invalid_parameter("page", "invalid page name"));
        }

        state.current_page = page.to_string();

        Ok(serde_json::json!({
            "result": format!("Switched to {} page", page),
            "previous_page": state.current_page
        }))
    }

    async fn create_timeline(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        let timeline = Timeline {
            name: name.to_string(),
            frame_rate: args["frame_rate"].as_str().map(|s| s.to_string()),
            resolution_width: args["resolution_width"].as_i64().map(|i| i as i32),
            resolution_height: args["resolution_height"].as_i64().map(|i| i as i32),
            markers: vec![],
        };

        state.timelines.insert(name.to_string(), timeline);
        state.current_timeline = Some(name.to_string());

        Ok(serde_json::json!({
            "result": format!("Created timeline '{}'", name),
            "timeline_id": Uuid::new_v4().to_string(),
            "frame_rate": args["frame_rate"],
            "resolution": format!("{}x{}",
                args["resolution_width"].as_i64().unwrap_or(1920),
                args["resolution_height"].as_i64().unwrap_or(1080)
            )
        }))
    }

    async fn add_marker(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        if state.current_timeline.is_none() {
            return Err(ResolveError::TimelineNotFound {
                name: "current".to_string(),
            });
        }

        let timeline_name = state.current_timeline.as_ref().unwrap();
        let timeline = state.timelines.get_mut(timeline_name).ok_or_else(|| {
            ResolveError::TimelineNotFound {
                name: timeline_name.clone(),
            }
        })?;

        let marker = Marker {
            frame: args["frame"].as_i64().map(|i| i as i32),
            color: args["color"].as_str().unwrap_or("Blue").to_string(),
            note: args["note"].as_str().unwrap_or("").to_string(),
        };

        timeline.markers.push(marker);

        Ok(serde_json::json!({
            "result": format!("Added {} marker to timeline '{}'",
                args["color"].as_str().unwrap_or("Blue"), timeline_name),
            "marker_id": Uuid::new_v4().to_string(),
            "total_markers": timeline.markers.len()
        }))
    }

    async fn import_media(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let file_path = args["file_path"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("file_path", "required string"))?;

        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        // Extract filename from path
        let filename = std::path::Path::new(file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown_file");

        let clip = Clip {
            name: filename.to_string(),
            file_path: file_path.to_string(),
            bin: None,
            linked: true,
            proxy_path: None,
        };

        state.media_pool.clips.insert(filename.to_string(), clip);

        Ok(serde_json::json!({
            "result": format!("Imported media: {}", filename),
            "clip_id": Uuid::new_v4().to_string(),
            "file_size": "simulated",
            "duration": "00:01:30:00"
        }))
    }

    async fn create_bin(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        // Check if bin already exists - if so, return success (idempotent operation)
        if state.media_pool.bins.contains_key(name) {
            return Ok(serde_json::json!({
                "result": format!("Bin '{}' already exists", name),
                "bin_id": Uuid::new_v4().to_string(),
                "already_existed": true
            }));
        }

        let bin = Bin {
            name: name.to_string(),
            clips: vec![],
        };

        state.media_pool.bins.insert(name.to_string(), bin);

        Ok(serde_json::json!({
            "result": format!("Created bin '{}'", name),
            "bin_id": Uuid::new_v4().to_string(),
            "already_existed": false
        }))
    }

    async fn auto_sync_audio(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_names = args["clip_names"]
            .as_array()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_names", "required array"))?;

        let sync_method = args["sync_method"].as_str().unwrap_or("waveform");
        let clips_found = clip_names.len();

        // Simulate sync processing
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Ok(serde_json::json!({
            "result": format!("Synchronized {} clips using {} method", clips_found, sync_method),
            "sync_id": Uuid::new_v4().to_string(),
            "processing_time": "1.2s"
        }))
    }

    async fn unlink_clips(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_names = args["clip_names"]
            .as_array()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_names", "required array"))?;

        Ok(serde_json::json!({
            "result": format!("Unlinked {} clips", clip_names.len()),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn relink_clips(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_names = args["clip_names"]
            .as_array()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_names", "required array"))?;

        Ok(serde_json::json!({
            "result": format!("Relinked {} clips", clip_names.len()),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn create_sub_clip(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        let start_frame = args["start_frame"].as_i64().unwrap_or(0) as i32;
        let end_frame = args["end_frame"].as_i64().unwrap_or(100) as i32;

        let default_sub_clip_name = format!("{}_subclip", clip_name);
        let sub_clip_name = args["sub_clip_name"]
            .as_str()
            .unwrap_or(&default_sub_clip_name);

        Ok(serde_json::json!({
            "result": format!("Created subclip '{}' from '{}' (frames {}-{})",
                sub_clip_name, clip_name, start_frame, end_frame),
            "subclip_id": Uuid::new_v4().to_string(),
            "duration_frames": end_frame - start_frame
        }))
    }

    async fn link_proxy_media(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;

        Ok(serde_json::json!({
            "result": format!("Linked proxy media for clip '{}'", clip_name),
            "proxy_id": Uuid::new_v4().to_string()
        }))
    }

    async fn unlink_proxy_media(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;

        Ok(serde_json::json!({
            "result": format!("Unlinked proxy media for clip '{}'", clip_name),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn replace_clip(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        let replacement_path = args["replacement_path"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("replacement_path", "required string")
        })?;

        Ok(serde_json::json!({
            "result": format!("Replaced clip '{}' with '{}'", clip_name, replacement_path),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn delete_timeline(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        if state.timelines.remove(name).is_none() {
            return Err(ResolveError::TimelineNotFound {
                name: name.to_string(),
            });
        }

        // Reset current timeline if it was the deleted one
        if state.current_timeline.as_ref() == Some(&name.to_string()) {
            state.current_timeline = None;
        }

        Ok(serde_json::json!({
            "result": format!("Deleted timeline '{}'", name),
            "remaining_timelines": state.timelines.len(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_current_timeline(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        if !state.timelines.contains_key(name) {
            return Err(ResolveError::TimelineNotFound {
                name: name.to_string(),
            });
        }

        state.current_timeline = Some(name.to_string());

        Ok(serde_json::json!({
            "result": format!("Set current timeline to '{}'", name),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn create_empty_timeline(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;

        // In simulation mode, auto-create a project if none exists
        if state.current_project.is_none() {
            match self.mode {
                ConnectionMode::Simulation => {
                    // Auto-create a default project in simulation mode
                    let default_project = "Default Project".to_string();
                    state.projects.push(default_project.clone());
                    state.current_project = Some(default_project);
                    tracing::info!("Auto-created default project for timeline creation");
                }
                ConnectionMode::Real => {
                    return Err(ResolveError::NotRunning);
                }
            }
        }

        let timeline = Timeline {
            name: name.to_string(),
            frame_rate: args["frame_rate"].as_str().map(|s| s.to_string()),
            resolution_width: args["resolution_width"].as_i64().map(|i| i as i32),
            resolution_height: args["resolution_height"].as_i64().map(|i| i as i32),
            markers: vec![],
        };

        state.timelines.insert(name.to_string(), timeline);
        state.current_timeline = Some(name.to_string());

        Ok(serde_json::json!({
            "result": format!("Created empty timeline '{}'", name),
            "timeline_id": Uuid::new_v4().to_string(),
            "frame_rate": args["frame_rate"],
            "resolution": format!("{}x{}",
                args["resolution_width"].as_i64().unwrap_or(1920),
                args["resolution_height"].as_i64().unwrap_or(1080)
            ),
            "video_tracks": args["video_tracks"].as_i64().unwrap_or(1),
            "audio_tracks": args["audio_tracks"].as_i64().unwrap_or(2)
        }))
    }

    async fn add_clip_to_timeline(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;

        let timeline_name = if let Some(name) = args["timeline_name"].as_str() {
            name.to_string()
        } else {
            state
                .current_timeline
                .clone()
                .ok_or_else(|| ResolveError::TimelineNotFound {
                    name: "current".to_string(),
                })?
        };

        if !state.timelines.contains_key(&timeline_name) {
            return Err(ResolveError::TimelineNotFound {
                name: timeline_name,
            });
        }

        if !state.media_pool.clips.contains_key(clip_name) {
            return Err(ResolveError::MediaNotFound {
                name: clip_name.to_string(),
            });
        }

        Ok(serde_json::json!({
            "result": format!("Added clip '{}' to timeline '{}'", clip_name, timeline_name),
            "timeline_item_id": Uuid::new_v4().to_string(),
            "track": "Video 1"
        }))
    }

    async fn list_timelines_tool(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let timeline_names: Vec<&String> = state.timelines.keys().collect();
        let timeline_list = if timeline_names.is_empty() {
            "No timelines available".to_string()
        } else {
            timeline_names
                .iter()
                .map(|&name| name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        };

        Ok(serde_json::json!({
            "result": format!("Timelines: {}", timeline_list),
            "count": timeline_names.len(),
            "current_timeline": state.current_timeline
        }))
    }

    async fn get_timeline_tracks(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = if let Some(name) = args["timeline_name"].as_str() {
            name.to_string()
        } else {
            state
                .current_timeline
                .clone()
                .ok_or_else(|| ResolveError::TimelineNotFound {
                    name: "current".to_string(),
                })?
        };

        if !state.timelines.contains_key(&timeline_name) {
            return Err(ResolveError::TimelineNotFound {
                name: timeline_name,
            });
        }

        // Simulate track information
        let video_tracks = vec!["Video 1", "Video 2", "Video 3"];
        let audio_tracks = vec!["Audio 1", "Audio 2", "Audio 3", "Audio 4"];

        Ok(serde_json::json!({
            "result": format!("Timeline '{}' tracks retrieved", timeline_name),
            "video_tracks": video_tracks,
            "audio_tracks": audio_tracks,
            "total_tracks": video_tracks.len() + audio_tracks.len()
        }))
    }

    // ==================== COLOR OPERATIONS (Phase 3 Week 3) ====================

    async fn apply_lut(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let lut_path = args["lut_path"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("lut_path", "required string"))?;
        let node_index = args["node_index"]
            .as_i64()
            .unwrap_or(state.color_state.current_node_index as i64) as i32;

        // Validate LUT exists (check if it's in our available LUTs or is a file path)
        let lut_name = if lut_path.starts_with('/') {
            // File path - validate it exists
            std::path::Path::new(lut_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown LUT")
                .to_string()
        } else {
            // Check if it's a known LUT
            if !state.color_state.available_luts.contains_key(lut_path) {
                return Err(ResolveError::FileNotFound {
                    path: lut_path.to_string(),
                });
            }
            lut_path.to_string()
        };

        // Apply LUT to current clip
        if let Some(clip_name) = &state.color_state.current_clip {
            let grade = state
                .color_state
                .clip_grades
                .entry(clip_name.clone())
                .or_default();
            grade.applied_luts.push(lut_name.clone());
        }

        Ok(serde_json::json!({
            "result": format!("Applied LUT '{}' to node {}", lut_name, node_index),
            "lut_path": lut_path,
            "node_index": node_index,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_color_wheel_param(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let wheel = args["wheel"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("wheel", "required string"))?;
        let param = args["param"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("param", "required string"))?;
        let value = args["value"]
            .as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("value", "required number"))?;
        let node_index = args["node_index"]
            .as_i64()
            .unwrap_or(state.color_state.current_node_index as i64) as i32;

        // Validate wheel and param
        let valid_wheels = vec!["lift", "gamma", "gain", "offset"];
        let valid_params = vec!["red", "green", "blue", "master"];

        if !valid_wheels.contains(&wheel) {
            return Err(ResolveError::invalid_parameter(
                "wheel",
                "must be lift, gamma, gain, or offset",
            ));
        }
        if !valid_params.contains(&param) {
            return Err(ResolveError::invalid_parameter(
                "param",
                "must be red, green, blue, or master",
            ));
        }

        // Apply to current clip
        if let Some(clip_name) = &state.color_state.current_clip {
            let grade = state
                .color_state
                .clip_grades
                .entry(clip_name.clone())
                .or_default();

            let wheel_params = match wheel {
                "lift" => &mut grade.lift,
                "gamma" => &mut grade.gamma,
                "gain" => &mut grade.gain,
                "offset" => &mut grade.offset,
                _ => unreachable!(),
            };

            match param {
                "red" => wheel_params.red = value,
                "green" => wheel_params.green = value,
                "blue" => wheel_params.blue = value,
                "master" => wheel_params.master = value,
                _ => unreachable!(),
            }
        }

        Ok(serde_json::json!({
            "result": format!("Set {} {} to {} on node {}", wheel, param, value, node_index),
            "wheel": wheel,
            "param": param,
            "value": value,
            "node_index": node_index,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn add_node(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let node_type = args["node_type"].as_str().unwrap_or("serial");
        let label = args["label"].as_str();

        // Validate node type
        let valid_types = vec!["serial", "parallel", "layer"];
        if !valid_types.contains(&node_type) {
            return Err(ResolveError::invalid_parameter(
                "node_type",
                "must be serial, parallel, or layer",
            ));
        }

        // Add node to current clip
        if let Some(clip_name) = &state.color_state.current_clip {
            let grade = state
                .color_state
                .clip_grades
                .entry(clip_name.clone())
                .or_default();
            grade.node_count += 1;

            if let Some(label_str) = label {
                grade
                    .node_labels
                    .insert(grade.node_count, label_str.to_string());
            }
        }

        let new_node_index = state.color_state.current_node_index + 1;
        state.color_state.current_node_index = new_node_index;

        Ok(serde_json::json!({
            "result": format!("Added {} node {}", node_type, new_node_index),
            "node_type": node_type,
            "node_index": new_node_index,
            "label": label,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn copy_grade(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let source_clip_name = args["source_clip_name"].as_str();
        let target_clip_name = args["target_clip_name"].as_str();
        let mode = args["mode"].as_str().unwrap_or("full");

        // Use current clip as source if not specified
        let source = if let Some(source) = source_clip_name {
            source.to_string()
        } else {
            state.color_state.current_clip.clone().ok_or_else(|| {
                ResolveError::invalid_parameter("source_clip_name", "no current clip")
            })?
        };

        // Use current clip as target if not specified
        let target = if let Some(target) = target_clip_name {
            target.to_string()
        } else {
            state.color_state.current_clip.clone().ok_or_else(|| {
                ResolveError::invalid_parameter("target_clip_name", "no current clip")
            })?
        };

        // Get source grade
        let source_grade = state
            .color_state
            .clip_grades
            .get(&source)
            .cloned()
            .unwrap_or_default();

        // Apply to target based on mode
        let result_msg = match mode {
            "full" => {
                state
                    .color_state
                    .clip_grades
                    .insert(target.clone(), source_grade);
                format!("Copied full grade from '{}' to '{}'", source, target)
            }
            "current_node" => {
                // Simulate copying current node only
                format!(
                    "Copied current node grade from '{}' to '{}'",
                    source, target
                )
            }
            "all_nodes" => {
                state
                    .color_state
                    .clip_grades
                    .insert(target.clone(), source_grade);
                format!("Copied all nodes from '{}' to '{}'", source, target)
            }
            _ => {
                return Err(ResolveError::invalid_parameter(
                    "mode",
                    "must be full, current_node, or all_nodes",
                ))
            }
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "source_clip": source,
            "target_clip": target,
            "mode": mode,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn save_color_preset(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str();
        let preset_name = args["preset_name"].as_str();
        let album_name = args["album_name"].as_str().unwrap_or("DaVinci Resolve");

        // Use current clip if not specified
        let source_clip =
            if let Some(clip) = clip_name {
                clip.to_string()
            } else {
                state.color_state.current_clip.clone().ok_or_else(|| {
                    ResolveError::invalid_parameter("clip_name", "no current clip")
                })?
            };

        // Use clip name as preset name if not specified
        let preset_name_final = if let Some(name) = preset_name {
            name.to_string()
        } else {
            format!("{}_preset", source_clip)
        };

        // Get clip grade
        let grade = state
            .color_state
            .clip_grades
            .get(&source_clip)
            .cloned()
            .unwrap_or_default();

        // Save preset
        let preset = ColorPreset {
            name: preset_name_final.clone(),
            album: album_name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            grade_data: grade,
        };

        state
            .color_state
            .color_presets
            .insert(preset_name_final.clone(), preset);

        Ok(serde_json::json!({
            "result": format!("Saved color preset '{}' from clip '{}' to album '{}'",
                preset_name_final, source_clip, album_name),
            "preset_name": preset_name_final,
            "album": album_name,
            "source_clip": source_clip,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn apply_color_preset(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_id = args["preset_id"].as_str();
        let preset_name = args["preset_name"].as_str();
        let clip_name = args["clip_name"].as_str();
        let album_name = args["album_name"].as_str().unwrap_or("DaVinci Resolve");

        // Find preset by ID or name
        let preset = if let Some(id) = preset_id {
            state.color_state.color_presets.get(id)
        } else if let Some(name) = preset_name {
            state.color_state.color_presets.get(name)
        } else {
            return Err(ResolveError::invalid_parameter(
                "preset_id or preset_name",
                "one is required",
            ));
        };

        let preset =
            preset.ok_or_else(|| ResolveError::invalid_parameter("preset", "preset not found"))?;

        // Use current clip if not specified
        let target_clip =
            if let Some(clip) = clip_name {
                clip.to_string()
            } else {
                state.color_state.current_clip.clone().ok_or_else(|| {
                    ResolveError::invalid_parameter("clip_name", "no current clip")
                })?
            };

        // Apply preset to clip
        state
            .color_state
            .clip_grades
            .insert(target_clip.clone(), preset.grade_data.clone());

        Ok(serde_json::json!({
            "result": format!("Applied color preset '{}' from album '{}' to clip '{}'",
                preset.name, album_name, target_clip),
            "preset_name": preset.name,
            "album": album_name,
            "target_clip": target_clip,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn delete_color_preset(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_id = args["preset_id"].as_str();
        let preset_name = args["preset_name"].as_str();
        let album_name = args["album_name"].as_str().unwrap_or("DaVinci Resolve");

        // Find preset by ID or name
        let preset_key = if let Some(id) = preset_id {
            id.to_string()
        } else if let Some(name) = preset_name {
            name.to_string()
        } else {
            return Err(ResolveError::invalid_parameter(
                "preset_id or preset_name",
                "one is required",
            ));
        };

        let removed_preset = state
            .color_state
            .color_presets
            .remove(&preset_key)
            .ok_or_else(|| ResolveError::invalid_parameter("preset", "preset not found"))?;

        Ok(serde_json::json!({
            "result": format!("Deleted color preset '{}' from album '{}'",
                removed_preset.name, album_name),
            "preset_name": removed_preset.name,
            "album": album_name,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn export_lut(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str();
        let export_path = args["export_path"].as_str();
        let lut_format = args["lut_format"].as_str().unwrap_or("Cube");
        let lut_size = args["lut_size"].as_str().unwrap_or("33Point");

        // Use current clip if not specified
        let source_clip =
            if let Some(clip) = clip_name {
                clip.to_string()
            } else {
                state.color_state.current_clip.clone().ok_or_else(|| {
                    ResolveError::invalid_parameter("clip_name", "no current clip")
                })?
            };

        // Validate format and size
        let valid_formats = vec!["Cube", "Davinci", "3dl", "Panasonic"];
        let valid_sizes = vec!["17Point", "33Point", "65Point"];

        if !valid_formats.contains(&lut_format) {
            return Err(ResolveError::invalid_parameter(
                "lut_format",
                "invalid format",
            ));
        }
        if !valid_sizes.contains(&lut_size) {
            return Err(ResolveError::invalid_parameter("lut_size", "invalid size"));
        }

        // Generate export path if not provided
        let final_export_path = if let Some(path) = export_path {
            path.to_string()
        } else {
            format!("/tmp/{}_grade.{}", source_clip, lut_format.to_lowercase())
        };

        Ok(serde_json::json!({
            "result": format!("Exported LUT from clip '{}' to '{}'", source_clip, final_export_path),
            "source_clip": source_clip,
            "export_path": final_export_path,
            "format": lut_format,
            "size": lut_size,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    // ==================== TIMELINE ITEM OPERATIONS (Phase 4 Week 1) ====================

    async fn set_timeline_item_transform(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_name = args["property_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let property_value = args["property_value"]
            .as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("property_value", "required number"))?;

        // Validate property name
        let valid_properties = vec![
            "Pan",
            "Tilt",
            "ZoomX",
            "ZoomY",
            "Rotation",
            "AnchorPointX",
            "AnchorPointY",
            "Pitch",
            "Yaw",
        ];
        if !valid_properties.contains(&property_name) {
            return Err(ResolveError::invalid_parameter(
                "property_name",
                "invalid transform property",
            ));
        }

        // Get or create timeline item
        let timeline_item = state
            .timeline_items
            .items
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| {
                state.timeline_items.item_counter += 1;
                TimelineItemState {
                    id: timeline_item_id.to_string(),
                    timeline_name: state.current_timeline.clone().unwrap_or_default(),
                    clip_name: format!("clip_{}", state.timeline_items.item_counter),
                    ..Default::default()
                }
            });

        // Set transform property
        match property_name {
            "Pan" => timeline_item.transform.pan = property_value,
            "Tilt" => timeline_item.transform.tilt = property_value,
            "ZoomX" => timeline_item.transform.zoom_x = property_value,
            "ZoomY" => timeline_item.transform.zoom_y = property_value,
            "Rotation" => timeline_item.transform.rotation = property_value,
            "AnchorPointX" => timeline_item.transform.anchor_point_x = property_value,
            "AnchorPointY" => timeline_item.transform.anchor_point_y = property_value,
            "Pitch" => timeline_item.transform.pitch = property_value,
            "Yaw" => timeline_item.transform.yaw = property_value,
            _ => unreachable!(),
        }

        Ok(serde_json::json!({
            "result": format!("Set {} to {} for timeline item '{}'", property_name, property_value, timeline_item_id),
            "timeline_item_id": timeline_item_id,
            "property_name": property_name,
            "property_value": property_value,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_crop(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let crop_type = args["crop_type"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("crop_type", "required string"))?;
        let crop_value = args["crop_value"]
            .as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("crop_value", "required number"))?;

        // Validate crop type and value
        let valid_crop_types = vec!["Left", "Right", "Top", "Bottom"];
        if !valid_crop_types.contains(&crop_type) {
            return Err(ResolveError::invalid_parameter(
                "crop_type",
                "must be Left, Right, Top, or Bottom",
            ));
        }
        if crop_value < 0.0 || crop_value > 1.0 {
            return Err(ResolveError::invalid_parameter(
                "crop_value",
                "must be between 0.0 and 1.0",
            ));
        }

        // Get or create timeline item
        let timeline_item = state
            .timeline_items
            .items
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| {
                state.timeline_items.item_counter += 1;
                TimelineItemState {
                    id: timeline_item_id.to_string(),
                    timeline_name: state.current_timeline.clone().unwrap_or_default(),
                    clip_name: format!("clip_{}", state.timeline_items.item_counter),
                    ..Default::default()
                }
            });

        // Set crop property
        match crop_type {
            "Left" => timeline_item.crop.left = crop_value,
            "Right" => timeline_item.crop.right = crop_value,
            "Top" => timeline_item.crop.top = crop_value,
            "Bottom" => timeline_item.crop.bottom = crop_value,
            _ => unreachable!(),
        }

        Ok(serde_json::json!({
            "result": format!("Set {} crop to {} for timeline item '{}'", crop_type, crop_value, timeline_item_id),
            "timeline_item_id": timeline_item_id,
            "crop_type": crop_type,
            "crop_value": crop_value,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_composite(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let composite_mode = args["composite_mode"].as_str();
        let opacity = args["opacity"].as_f64();

        // Validate composite mode if provided
        if let Some(mode) = composite_mode {
            let valid_modes = vec![
                "Normal",
                "Add",
                "Multiply",
                "Screen",
                "Overlay",
                "SoftLight",
                "HardLight",
                "ColorDodge",
                "ColorBurn",
                "Darken",
                "Lighten",
                "Difference",
                "Exclusion",
            ];
            if !valid_modes.contains(&mode) {
                return Err(ResolveError::invalid_parameter(
                    "composite_mode",
                    "invalid composite mode",
                ));
            }
        }

        // Validate opacity if provided
        if let Some(opacity_val) = opacity {
            if opacity_val < 0.0 || opacity_val > 1.0 {
                return Err(ResolveError::invalid_parameter(
                    "opacity",
                    "must be between 0.0 and 1.0",
                ));
            }
        }

        // Get or create timeline item
        let timeline_item = state
            .timeline_items
            .items
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| {
                state.timeline_items.item_counter += 1;
                TimelineItemState {
                    id: timeline_item_id.to_string(),
                    timeline_name: state.current_timeline.clone().unwrap_or_default(),
                    clip_name: format!("clip_{}", state.timeline_items.item_counter),
                    composite: CompositeProperties {
                        mode: "Normal".to_string(),
                        opacity: 1.0,
                    },
                    ..Default::default()
                }
            });

        // Set composite properties
        let mut result_parts = Vec::new();
        if let Some(mode) = composite_mode {
            timeline_item.composite.mode = mode.to_string();
            result_parts.push(format!("composite mode to {}", mode));
        }
        if let Some(opacity_val) = opacity {
            timeline_item.composite.opacity = opacity_val;
            result_parts.push(format!("opacity to {}", opacity_val));
        }

        let result_msg = if result_parts.is_empty() {
            "No composite properties changed".to_string()
        } else {
            format!(
                "Set {} for timeline item '{}'",
                result_parts.join(" and "),
                timeline_item_id
            )
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "composite_mode": composite_mode,
            "opacity": opacity,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_retime(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let speed = args["speed"].as_f64();
        let process = args["process"].as_str();

        // Validate speed if provided
        if let Some(speed_val) = speed {
            if speed_val <= 0.0 || speed_val > 10.0 {
                return Err(ResolveError::invalid_parameter(
                    "speed",
                    "must be between 0.0 and 10.0",
                ));
            }
        }

        // Validate process if provided
        if let Some(process_str) = process {
            let valid_processes = vec!["NearestFrame", "FrameBlend", "OpticalFlow"];
            if !valid_processes.contains(&process_str) {
                return Err(ResolveError::invalid_parameter(
                    "process",
                    "must be NearestFrame, FrameBlend, or OpticalFlow",
                ));
            }
        }

        // Get or create timeline item
        let timeline_item = state
            .timeline_items
            .items
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| {
                state.timeline_items.item_counter += 1;
                TimelineItemState {
                    id: timeline_item_id.to_string(),
                    timeline_name: state.current_timeline.clone().unwrap_or_default(),
                    clip_name: format!("clip_{}", state.timeline_items.item_counter),
                    retime: RetimeProperties {
                        speed: 1.0,
                        process: "NearestFrame".to_string(),
                    },
                    ..Default::default()
                }
            });

        // Set retime properties
        let mut result_parts = Vec::new();
        if let Some(speed_val) = speed {
            timeline_item.retime.speed = speed_val;
            result_parts.push(format!("speed to {}x", speed_val));
        }
        if let Some(process_str) = process {
            timeline_item.retime.process = process_str.to_string();
            result_parts.push(format!("process to {}", process_str));
        }

        let result_msg = if result_parts.is_empty() {
            "No retime properties changed".to_string()
        } else {
            format!(
                "Set {} for timeline item '{}'",
                result_parts.join(" and "),
                timeline_item_id
            )
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "speed": speed,
            "process": process,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_stabilization(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let enabled = args["enabled"].as_bool();
        let method = args["method"].as_str();
        let strength = args["strength"].as_f64();

        // Validate method if provided
        if let Some(method_str) = method {
            let valid_methods = vec!["Perspective", "Similarity", "Translation"];
            if !valid_methods.contains(&method_str) {
                return Err(ResolveError::invalid_parameter(
                    "method",
                    "must be Perspective, Similarity, or Translation",
                ));
            }
        }

        // Validate strength if provided
        if let Some(strength_val) = strength {
            if strength_val < 0.0 || strength_val > 1.0 {
                return Err(ResolveError::invalid_parameter(
                    "strength",
                    "must be between 0.0 and 1.0",
                ));
            }
        }

        // Get or create timeline item
        let timeline_item = state
            .timeline_items
            .items
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| {
                state.timeline_items.item_counter += 1;
                TimelineItemState {
                    id: timeline_item_id.to_string(),
                    timeline_name: state.current_timeline.clone().unwrap_or_default(),
                    clip_name: format!("clip_{}", state.timeline_items.item_counter),
                    stabilization: StabilizationProperties {
                        enabled: false,
                        method: "Perspective".to_string(),
                        strength: 0.5,
                    },
                    ..Default::default()
                }
            });

        // Set stabilization properties
        let mut result_parts = Vec::new();
        if let Some(enabled_val) = enabled {
            timeline_item.stabilization.enabled = enabled_val;
            result_parts.push(format!("enabled to {}", enabled_val));
        }
        if let Some(method_str) = method {
            timeline_item.stabilization.method = method_str.to_string();
            result_parts.push(format!("method to {}", method_str));
        }
        if let Some(strength_val) = strength {
            timeline_item.stabilization.strength = strength_val;
            result_parts.push(format!("strength to {}", strength_val));
        }

        let result_msg = if result_parts.is_empty() {
            "No stabilization properties changed".to_string()
        } else {
            format!(
                "Set stabilization {} for timeline item '{}'",
                result_parts.join(", "),
                timeline_item_id
            )
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "enabled": enabled,
            "method": method,
            "strength": strength,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_audio(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let volume = args["volume"].as_f64();
        let pan = args["pan"].as_f64();
        let eq_enabled = args["eq_enabled"].as_bool();

        // Validate volume if provided
        if let Some(volume_val) = volume {
            if volume_val < 0.0 || volume_val > 2.0 {
                return Err(ResolveError::invalid_parameter(
                    "volume",
                    "must be between 0.0 and 2.0",
                ));
            }
        }

        // Validate pan if provided
        if let Some(pan_val) = pan {
            if pan_val < -1.0 || pan_val > 1.0 {
                return Err(ResolveError::invalid_parameter(
                    "pan",
                    "must be between -1.0 and 1.0",
                ));
            }
        }

        // Get or create timeline item
        let timeline_item = state
            .timeline_items
            .items
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| {
                state.timeline_items.item_counter += 1;
                TimelineItemState {
                    id: timeline_item_id.to_string(),
                    timeline_name: state.current_timeline.clone().unwrap_or_default(),
                    clip_name: format!("clip_{}", state.timeline_items.item_counter),
                    audio: AudioProperties {
                        volume: 1.0,
                        pan: 0.0,
                        eq_enabled: false,
                    },
                    ..Default::default()
                }
            });

        // Set audio properties
        let mut result_parts = Vec::new();
        if let Some(volume_val) = volume {
            timeline_item.audio.volume = volume_val;
            result_parts.push(format!("volume to {}", volume_val));
        }
        if let Some(pan_val) = pan {
            timeline_item.audio.pan = pan_val;
            result_parts.push(format!("pan to {}", pan_val));
        }
        if let Some(eq_val) = eq_enabled {
            timeline_item.audio.eq_enabled = eq_val;
            result_parts.push(format!("EQ enabled to {}", eq_val));
        }

        let result_msg = if result_parts.is_empty() {
            "No audio properties changed".to_string()
        } else {
            format!(
                "Set audio {} for timeline item '{}'",
                result_parts.join(", "),
                timeline_item_id
            )
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "volume": volume,
            "pan": pan,
            "eq_enabled": eq_enabled,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn get_timeline_item_properties(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;

        // Get timeline item
        let timeline_item = state
            .timeline_items
            .items
            .get(timeline_item_id)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("timeline_item_id", "timeline item not found")
            })?;

        Ok(serde_json::json!({
            "result": format!("Retrieved properties for timeline item '{}'", timeline_item_id),
            "timeline_item_id": timeline_item_id,
            "timeline_name": timeline_item.timeline_name,
            "clip_name": timeline_item.clip_name,
            "properties": {
                "transform": {
                    "pan": timeline_item.transform.pan,
                    "tilt": timeline_item.transform.tilt,
                    "zoom_x": timeline_item.transform.zoom_x,
                    "zoom_y": timeline_item.transform.zoom_y,
                    "rotation": timeline_item.transform.rotation,
                    "anchor_point_x": timeline_item.transform.anchor_point_x,
                    "anchor_point_y": timeline_item.transform.anchor_point_y,
                    "pitch": timeline_item.transform.pitch,
                    "yaw": timeline_item.transform.yaw
                },
                "crop": {
                    "left": timeline_item.crop.left,
                    "right": timeline_item.crop.right,
                    "top": timeline_item.crop.top,
                    "bottom": timeline_item.crop.bottom
                },
                "composite": {
                    "mode": timeline_item.composite.mode,
                    "opacity": timeline_item.composite.opacity
                },
                "retime": {
                    "speed": timeline_item.retime.speed,
                    "process": timeline_item.retime.process
                },
                "stabilization": {
                    "enabled": timeline_item.stabilization.enabled,
                    "method": timeline_item.stabilization.method,
                    "strength": timeline_item.stabilization.strength
                },
                "audio": {
                    "volume": timeline_item.audio.volume,
                    "pan": timeline_item.audio.pan,
                    "eq_enabled": timeline_item.audio.eq_enabled
                }
            },
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn reset_timeline_item_properties(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_type = args["property_type"].as_str();

        // Get timeline item
        let timeline_item = state
            .timeline_items
            .items
            .get_mut(timeline_item_id)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("timeline_item_id", "timeline item not found")
            })?;

        let mut reset_parts = Vec::new();

        // Reset specific property type or all if not specified
        match property_type {
            Some("transform") => {
                timeline_item.transform = TransformProperties::default();
                reset_parts.push("transform");
            }
            Some("crop") => {
                timeline_item.crop = CropProperties::default();
                reset_parts.push("crop");
            }
            Some("composite") => {
                timeline_item.composite = CompositeProperties {
                    mode: "Normal".to_string(),
                    opacity: 1.0,
                };
                reset_parts.push("composite");
            }
            Some("retime") => {
                timeline_item.retime = RetimeProperties {
                    speed: 1.0,
                    process: "NearestFrame".to_string(),
                };
                reset_parts.push("retime");
            }
            Some("stabilization") => {
                timeline_item.stabilization = StabilizationProperties::default();
                reset_parts.push("stabilization");
            }
            Some("audio") => {
                timeline_item.audio = AudioProperties {
                    volume: 1.0,
                    pan: 0.0,
                    eq_enabled: false,
                };
                reset_parts.push("audio");
            }
            Some(_invalid_type) => {
                return Err(ResolveError::invalid_parameter(
                    "property_type",
                    "must be transform, crop, composite, retime, stabilization, or audio",
                ));
            }
            None => {
                // Reset all properties
                timeline_item.transform = TransformProperties::default();
                timeline_item.crop = CropProperties::default();
                timeline_item.composite = CompositeProperties {
                    mode: "Normal".to_string(),
                    opacity: 1.0,
                };
                timeline_item.retime = RetimeProperties {
                    speed: 1.0,
                    process: "NearestFrame".to_string(),
                };
                timeline_item.stabilization = StabilizationProperties::default();
                timeline_item.audio = AudioProperties {
                    volume: 1.0,
                    pan: 0.0,
                    eq_enabled: false,
                };
                reset_parts.push("all properties");
            }
        }

        let result_msg = format!(
            "Reset {} for timeline item '{}'",
            reset_parts.join(", "),
            timeline_item_id
        );

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "property_type": property_type,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    // ==================== KEYFRAME ANIMATION OPERATIONS (Phase 4 Week 2) ====================

    async fn add_keyframe(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_name = args["property_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"]
            .as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))?
            as i32;
        let value = args["value"]
            .as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("value", "required number"))?;

        // Validate property name
        let valid_properties = vec![
            "Pan",
            "Tilt",
            "ZoomX",
            "ZoomY",
            "Rotation",
            "AnchorPointX",
            "AnchorPointY",
            "Pitch",
            "Yaw",
            "Left",
            "Right",
            "Top",
            "Bottom",
            "Opacity",
            "Speed",
            "Strength",
            "Volume",
            "AudioPan",
        ];
        if !valid_properties.contains(&property_name) {
            return Err(ResolveError::invalid_parameter(
                "property_name",
                "must be a valid timeline item property",
            ));
        }

        // Validate frame position
        if frame < 0 {
            return Err(ResolveError::invalid_parameter(
                "frame",
                "must be non-negative",
            ));
        }

        // Generate keyframe ID
        state.keyframe_state.keyframe_counter += 1;
        let keyframe_id = state.keyframe_state.keyframe_counter;

        // Get or create timeline item keyframes
        let timeline_item_keyframes = state
            .keyframe_state
            .timeline_item_keyframes
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| TimelineItemKeyframes {
                timeline_item_id: timeline_item_id.to_string(),
                property_keyframes: HashMap::new(),
                keyframe_modes: KeyframeModes::default(),
            });

        // Create new keyframe
        let keyframe = Keyframe {
            id: keyframe_id,
            frame,
            value,
            interpolation: InterpolationType::Linear,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        // Add keyframe to property
        let property_keyframes = timeline_item_keyframes
            .property_keyframes
            .entry(property_name.to_string())
            .or_insert_with(Vec::new);

        // Insert keyframe in sorted order by frame
        let insert_pos = property_keyframes
            .binary_search_by_key(&frame, |k| k.frame)
            .unwrap_or_else(|pos| pos);
        property_keyframes.insert(insert_pos, keyframe);

        Ok(serde_json::json!({
            "result": format!("Added keyframe for '{}' at frame {} with value {}",
                property_name, frame, value),
            "timeline_item_id": timeline_item_id,
            "property_name": property_name,
            "frame": frame,
            "value": value,
            "keyframe_id": keyframe_id,
            "total_keyframes": property_keyframes.len(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn modify_keyframe(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_name = args["property_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"]
            .as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))?
            as i32;
        let new_value = args["new_value"].as_f64();
        let new_frame = args["new_frame"].as_i64().map(|f| f as i32);

        // Get timeline item keyframes
        let timeline_item_keyframes = state
            .keyframe_state
            .timeline_item_keyframes
            .get_mut(timeline_item_id)
            .ok_or_else(|| {
                ResolveError::invalid_parameter(
                    "timeline_item_id",
                    "no keyframes found for timeline item",
                )
            })?;

        // Get property keyframes
        let property_keyframes = timeline_item_keyframes
            .property_keyframes
            .get_mut(property_name)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("property_name", "no keyframes found for property")
            })?;

        // Find keyframe at specified frame
        let keyframe_index = property_keyframes
            .iter()
            .position(|k| k.frame == frame)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("frame", "no keyframe found at specified frame")
            })?;

        let mut modifications = Vec::new();

        // Modify value if provided
        if let Some(value) = new_value {
            property_keyframes[keyframe_index].value = value;
            modifications.push(format!("value to {}", value));
        }

        // Modify frame position if provided
        if let Some(new_frame_pos) = new_frame {
            if new_frame_pos < 0 {
                return Err(ResolveError::invalid_parameter(
                    "new_frame",
                    "must be non-negative",
                ));
            }

            // Remove keyframe from current position
            let mut keyframe = property_keyframes.remove(keyframe_index);
            keyframe.frame = new_frame_pos;

            // Re-insert in sorted order
            let insert_pos = property_keyframes
                .binary_search_by_key(&new_frame_pos, |k| k.frame)
                .unwrap_or_else(|pos| pos);
            property_keyframes.insert(insert_pos, keyframe);

            modifications.push(format!("frame to {}", new_frame_pos));
        }

        let result_msg = if modifications.is_empty() {
            "No modifications made to keyframe".to_string()
        } else {
            format!("Modified keyframe: {}", modifications.join(", "))
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "property_name": property_name,
            "original_frame": frame,
            "new_value": new_value,
            "new_frame": new_frame,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn delete_keyframe(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_name = args["property_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"]
            .as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))?
            as i32;

        // Get timeline item keyframes
        let timeline_item_keyframes = state
            .keyframe_state
            .timeline_item_keyframes
            .get_mut(timeline_item_id)
            .ok_or_else(|| {
                ResolveError::invalid_parameter(
                    "timeline_item_id",
                    "no keyframes found for timeline item",
                )
            })?;

        // Get property keyframes
        let property_keyframes = timeline_item_keyframes
            .property_keyframes
            .get_mut(property_name)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("property_name", "no keyframes found for property")
            })?;

        // Find and remove keyframe at specified frame
        let keyframe_index = property_keyframes
            .iter()
            .position(|k| k.frame == frame)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("frame", "no keyframe found at specified frame")
            })?;

        let deleted_keyframe = property_keyframes.remove(keyframe_index);

        Ok(serde_json::json!({
            "result": format!("Deleted keyframe for '{}' at frame {}", property_name, frame),
            "timeline_item_id": timeline_item_id,
            "property_name": property_name,
            "frame": frame,
            "deleted_value": deleted_keyframe.value,
            "remaining_keyframes": property_keyframes.len(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_keyframe_interpolation(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_name = args["property_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"]
            .as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))?
            as i32;
        let interpolation_type = args["interpolation_type"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("interpolation_type", "required string")
        })?;

        // Validate interpolation type
        let interpolation = match interpolation_type {
            "Linear" => InterpolationType::Linear,
            "Bezier" => InterpolationType::Bezier,
            "Ease-In" => InterpolationType::EaseIn,
            "Ease-Out" => InterpolationType::EaseOut,
            "Hold" => InterpolationType::Hold,
            _ => {
                return Err(ResolveError::invalid_parameter(
                    "interpolation_type",
                    "must be Linear, Bezier, Ease-In, Ease-Out, or Hold",
                ))
            }
        };

        // Get timeline item keyframes
        let timeline_item_keyframes = state
            .keyframe_state
            .timeline_item_keyframes
            .get_mut(timeline_item_id)
            .ok_or_else(|| {
                ResolveError::invalid_parameter(
                    "timeline_item_id",
                    "no keyframes found for timeline item",
                )
            })?;

        // Get property keyframes
        let property_keyframes = timeline_item_keyframes
            .property_keyframes
            .get_mut(property_name)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("property_name", "no keyframes found for property")
            })?;

        // Find keyframe at specified frame
        let keyframe = property_keyframes
            .iter_mut()
            .find(|k| k.frame == frame)
            .ok_or_else(|| {
                ResolveError::invalid_parameter("frame", "no keyframe found at specified frame")
            })?;

        keyframe.interpolation = interpolation;

        Ok(serde_json::json!({
            "result": format!("Set interpolation to '{}' for keyframe at frame {}",
                interpolation_type, frame),
            "timeline_item_id": timeline_item_id,
            "property_name": property_name,
            "frame": frame,
            "interpolation_type": interpolation_type,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn enable_keyframes(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let keyframe_mode = args["keyframe_mode"].as_str().unwrap_or("All");

        // Validate keyframe mode
        if !["All", "Color", "Sizing"].contains(&keyframe_mode) {
            return Err(ResolveError::invalid_parameter(
                "keyframe_mode",
                "must be All, Color, or Sizing",
            ));
        }

        // Get or create timeline item keyframes
        let timeline_item_keyframes = state
            .keyframe_state
            .timeline_item_keyframes
            .entry(timeline_item_id.to_string())
            .or_insert_with(|| TimelineItemKeyframes {
                timeline_item_id: timeline_item_id.to_string(),
                property_keyframes: HashMap::new(),
                keyframe_modes: KeyframeModes::default(),
            });

        // Set keyframe mode
        match keyframe_mode {
            "All" => timeline_item_keyframes.keyframe_modes.all_enabled = true,
            "Color" => timeline_item_keyframes.keyframe_modes.color_enabled = true,
            "Sizing" => timeline_item_keyframes.keyframe_modes.sizing_enabled = true,
            _ => unreachable!(),
        }

        Ok(serde_json::json!({
            "result": format!("Enabled '{}' keyframe mode for timeline item '{}'",
                keyframe_mode, timeline_item_id),
            "timeline_item_id": timeline_item_id,
            "keyframe_mode": keyframe_mode,
            "modes": {
                "all_enabled": timeline_item_keyframes.keyframe_modes.all_enabled,
                "color_enabled": timeline_item_keyframes.keyframe_modes.color_enabled,
                "sizing_enabled": timeline_item_keyframes.keyframe_modes.sizing_enabled
            },
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn get_keyframes(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "required string")
        })?;
        let property_name = args["property_name"].as_str();

        // Get timeline item keyframes
        let timeline_item_keyframes = state
            .keyframe_state
            .timeline_item_keyframes
            .get(timeline_item_id)
            .ok_or_else(|| {
                ResolveError::invalid_parameter(
                    "timeline_item_id",
                    "no keyframes found for timeline item",
                )
            })?;

        let mut result = serde_json::json!({
            "result": format!("Retrieved keyframes for timeline item '{}'", timeline_item_id),
            "timeline_item_id": timeline_item_id,
            "keyframe_modes": {
                "all_enabled": timeline_item_keyframes.keyframe_modes.all_enabled,
                "color_enabled": timeline_item_keyframes.keyframe_modes.color_enabled,
                "sizing_enabled": timeline_item_keyframes.keyframe_modes.sizing_enabled
            },
            "operation_id": Uuid::new_v4().to_string()
        });

        // If specific property requested, return only that property's keyframes
        if let Some(prop_name) = property_name {
            if let Some(keyframes) = timeline_item_keyframes.property_keyframes.get(prop_name) {
                let keyframe_data: Vec<serde_json::Value> = keyframes
                    .iter()
                    .map(|kf| {
                        serde_json::json!({
                            "id": kf.id,
                            "frame": kf.frame,
                            "value": kf.value,
                            "interpolation": format!("{:?}", kf.interpolation),
                            "created_at": kf.created_at
                        })
                    })
                    .collect();

                result["property_name"] = serde_json::Value::String(prop_name.to_string());
                result["keyframes"] = serde_json::Value::Array(keyframe_data);
                result["total_keyframes"] =
                    serde_json::Value::Number(serde_json::Number::from(keyframes.len()));
            } else {
                result["property_name"] = serde_json::Value::String(prop_name.to_string());
                result["keyframes"] = serde_json::Value::Array(vec![]);
                result["total_keyframes"] = serde_json::Value::Number(serde_json::Number::from(0));
            }
        } else {
            // Return all properties and their keyframes
            let mut all_properties = serde_json::Map::new();
            let mut total_count = 0;

            for (prop_name, keyframes) in &timeline_item_keyframes.property_keyframes {
                let keyframe_data: Vec<serde_json::Value> = keyframes
                    .iter()
                    .map(|kf| {
                        serde_json::json!({
                            "id": kf.id,
                            "frame": kf.frame,
                            "value": kf.value,
                            "interpolation": format!("{:?}", kf.interpolation),
                            "created_at": kf.created_at
                        })
                    })
                    .collect();

                all_properties.insert(prop_name.clone(), serde_json::Value::Array(keyframe_data));
                total_count += keyframes.len();
            }

            result["properties"] = serde_json::Value::Object(all_properties);
            result["total_keyframes"] =
                serde_json::Value::Number(serde_json::Number::from(total_count));
        }

        Ok(result)
    }

    // ==================== RENDER & DELIVERY OPERATIONS (Phase 4 Week 3) ====================

    async fn add_to_render_queue(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("preset_name", "required string"))?;
        let timeline_name = args["timeline_name"].as_str().unwrap_or_else(|| {
            state
                .current_timeline
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("Timeline 1")
        });
        let use_in_out_range = args["use_in_out_range"].as_bool().unwrap_or(false);

        // Validate timeline exists
        if !state.timelines.contains_key(timeline_name) {
            return Err(ResolveError::TimelineNotFound {
                name: timeline_name.to_string(),
            });
        }

        // Initialize default presets if none exist
        if state.render_state.render_presets.is_empty() {
            let default_preset = RenderPreset {
                name: "H.264 1080p".to_string(),
                format: "MP4".to_string(),
                codec: "H.264".to_string(),
                resolution: (1920, 1080),
                frame_rate: 24.0,
                quality: RenderQuality::High,
                audio_codec: "AAC".to_string(),
                audio_bitrate: 192,
                created_at: chrono::Utc::now(),
            };
            state
                .render_state
                .render_presets
                .insert("H.264 1080p".to_string(), default_preset);
        }

        // Validate preset exists
        if !state.render_state.render_presets.contains_key(preset_name) {
            return Err(ResolveError::PresetNotFound {
                name: preset_name.to_string(),
            });
        }

        // Generate job ID and output path
        state.render_state.job_counter += 1;
        let job_id = format!("job_{}", state.render_state.job_counter);
        let output_path = format!("/tmp/renders/{}_{}.mp4", timeline_name, job_id);

        // Create render job
        let render_job = RenderJob {
            id: job_id.clone(),
            timeline_name: timeline_name.to_string(),
            preset_name: preset_name.to_string(),
            output_path: output_path.clone(),
            use_in_out_range,
            created_at: chrono::Utc::now(),
            status: RenderJobStatus::Queued,
        };

        // Add to queue
        state.render_state.render_queue.push(render_job);

        Ok(serde_json::json!({
            "result": format!("Added timeline '{}' to render queue with preset '{}'", timeline_name, preset_name),
            "job_id": job_id,
            "timeline_name": timeline_name,
            "preset_name": preset_name,
            "output_path": output_path,
            "use_in_out_range": use_in_out_range,
            "queue_position": state.render_state.render_queue.len(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn start_render(&self, state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
        if state.render_state.render_queue.is_empty() {
            return Err(ResolveError::invalid_parameter(
                "render_queue",
                "no jobs in queue",
            ));
        }

        let mut started_jobs = Vec::new();
        let now = chrono::Utc::now();

        // Process all queued jobs
        for job in &mut state.render_state.render_queue {
            if matches!(job.status, RenderJobStatus::Queued) {
                job.status = RenderJobStatus::Rendering;

                // Create render progress tracking
                let progress = RenderProgress {
                    job_id: job.id.clone(),
                    progress_percent: 0.0,
                    estimated_time_remaining: Some(std::time::Duration::from_secs(120)),
                    current_frame: 0,
                    total_frames: 1000, // Simulated frame count
                    status_message: "Starting render...".to_string(),
                    last_update: now,
                };

                state
                    .render_state
                    .active_renders
                    .insert(job.id.clone(), progress);
                started_jobs.push(job.id.clone());
            }
        }

        if started_jobs.is_empty() {
            return Err(ResolveError::invalid_parameter(
                "render_queue",
                "no queued jobs to start",
            ));
        }

        tracing::info!("Started {} render jobs", started_jobs.len());

        Ok(serde_json::json!({
            "result": format!("Started {} render jobs", started_jobs.len()),
            "started_jobs": started_jobs,
            "total_active_renders": state.render_state.active_renders.len(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn clear_render_queue(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let queue_size = state.render_state.render_queue.len();
        let active_renders = state.render_state.active_renders.len();

        // Clear render queue and active renders
        state.render_state.render_queue.clear();
        state.render_state.active_renders.clear();

        tracing::info!(
            "Cleared render queue ({} jobs) and active renders ({} jobs)",
            queue_size,
            active_renders
        );

        Ok(serde_json::json!({
            "result": format!("Cleared render queue ({} jobs) and stopped {} active renders", queue_size, active_renders),
            "cleared_queue_jobs": queue_size,
            "stopped_active_renders": active_renders,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn get_render_status(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let queue_size = state.render_state.render_queue.len();
        let active_renders = state.render_state.active_renders.len();
        let completed_renders = state.render_state.render_history.len();

        // Collect active render details
        let active_render_details: Vec<_> = state.render_state.active_renders.values()
            .map(|progress| serde_json::json!({
                "job_id": progress.job_id,
                "progress_percent": progress.progress_percent,
                "current_frame": progress.current_frame,
                "total_frames": progress.total_frames,
                "status_message": progress.status_message,
                "estimated_time_remaining_seconds": progress.estimated_time_remaining.map(|d| d.as_secs())
            }))
            .collect();

        // Collect queued job details
        let queued_job_details: Vec<_> = state
            .render_state
            .render_queue
            .iter()
            .filter(|job| matches!(job.status, RenderJobStatus::Queued))
            .map(|job| {
                serde_json::json!({
                    "job_id": job.id,
                    "timeline_name": job.timeline_name,
                    "preset_name": job.preset_name,
                    "output_path": job.output_path,
                    "use_in_out_range": job.use_in_out_range
                })
            })
            .collect();

        Ok(serde_json::json!({
            "result": format!("Render status: {} queued, {} active, {} completed", queue_size, active_renders, completed_renders),
            "queued_jobs": queued_job_details.len(),
            "active_renders": active_render_details.len(),
            "completed_renders": completed_renders,
            "queued_job_details": queued_job_details,
            "active_render_details": active_render_details,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn export_project(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let export_path = args["export_path"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("export_path", "required string"))?;
        let include_media = args["include_media"].as_bool().unwrap_or(false);
        let project_name = args["project_name"].as_str().unwrap_or_else(|| {
            state
                .current_project
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("Unknown Project")
        });

        // Validate current project exists
        if state.current_project.is_none() {
            return Err(ResolveError::invalid_parameter(
                "project",
                "no project currently open",
            ));
        }

        // Validate export path
        if export_path.is_empty() {
            return Err(ResolveError::invalid_parameter(
                "export_path",
                "cannot be empty",
            ));
        }

        tracing::info!("Exporting project '{}' to '{}'", project_name, export_path);

        // Simulate export process
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Simulate export file size
        let timeline_count = state.timelines.len();
        let media_count = state.media_pool.clips.len();
        let estimated_size_mb = if include_media {
            500 + media_count * 50
        } else {
            50 + timeline_count * 10
        };

        Ok(serde_json::json!({
            "result": format!("Project '{}' exported successfully to '{}'", project_name, export_path),
            "project_name": project_name,
            "export_path": export_path,
            "include_media": include_media,
            "timeline_count": timeline_count,
            "media_count": media_count,
            "estimated_size_mb": estimated_size_mb,
            "export_timestamp": chrono::Utc::now().to_rfc3339(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn create_render_preset(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("preset_name", "required string"))?;
        let format = args["format"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("format", "required string"))?;
        let codec = args["codec"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("codec", "required string"))?;
        let resolution = (
            args["resolution_width"].as_i64().unwrap() as u32,
            args["resolution_height"].as_i64().unwrap() as u32,
        );
        let frame_rate = args["frame_rate"].as_f64().unwrap() as f32;
        let quality = args["quality"].as_u64().unwrap() as u32;
        let audio_codec = args["audio_codec"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("audio_codec", "required string"))?;
        let audio_bitrate = args["audio_bitrate"].as_u64().unwrap() as u32;

        // Validate format
        let valid_formats = vec!["MP4", "MOV", "MXF"];
        if !valid_formats.contains(&format) {
            return Err(ResolveError::invalid_parameter("format", "invalid format"));
        }

        // Validate codec
        let valid_codecs = vec!["H.264", "H.265", "ProRes"];
        if !valid_codecs.contains(&codec) {
            return Err(ResolveError::invalid_parameter("codec", "invalid codec"));
        }

        // Validate resolution
        if resolution.0 < 1920 || resolution.1 < 1080 {
            return Err(ResolveError::invalid_parameter(
                "resolution",
                "must be at least 1920x1080",
            ));
        }

        // Validate frame rate
        if frame_rate < 24.0 || frame_rate > 60.0 {
            return Err(ResolveError::invalid_parameter(
                "frame_rate",
                "must be between 24.0 and 60.0",
            ));
        }

        // Validate quality
        if quality < 1 || quality > 100 {
            return Err(ResolveError::invalid_parameter(
                "quality",
                "must be between 1 and 100",
            ));
        }

        // Validate audio codec
        let valid_audio_codecs = vec!["AAC", "ProRes"];
        if !valid_audio_codecs.contains(&audio_codec) {
            return Err(ResolveError::invalid_parameter(
                "audio_codec",
                "invalid audio codec",
            ));
        }

        // Validate audio bitrate
        if audio_bitrate < 64000 || audio_bitrate > 192000 {
            return Err(ResolveError::invalid_parameter(
                "audio_bitrate",
                "must be between 64kbps and 192kbps",
            ));
        }

        // Create new render preset
        let render_preset = RenderPreset {
            name: preset_name.to_string(),
            format: format.to_string(),
            codec: codec.to_string(),
            resolution,
            frame_rate,
            quality: RenderQuality::Custom(quality),
            audio_codec: audio_codec.to_string(),
            audio_bitrate,
            created_at: chrono::Utc::now(),
        };

        // Add preset to render presets
        state
            .render_state
            .render_presets
            .insert(preset_name.to_string(), render_preset);

        Ok(serde_json::json!({
            "result": format!("Created render preset '{}'", preset_name),
            "preset_name": preset_name,
            "format": format,
            "codec": codec,
            "resolution": format!("{}x{}", resolution.0, resolution.1),
            "frame_rate": frame_rate,
            "quality": quality,
            "audio_codec": audio_codec,
            "audio_bitrate": audio_bitrate,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    // ---- Project Management Operations ----
    async fn save_project(&self, state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        let project_name = state.current_project.as_ref().unwrap();

        // Simulate save operation
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        Ok(serde_json::json!({
            "result": format!("Saved project '{}'", project_name),
            "operation_id": Uuid::new_v4().to_string(),
            "save_time": chrono::Utc::now().to_rfc3339()
        }))
    }

    async fn close_project(&self, state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        let project_name = state.current_project.take().unwrap();

        // Reset project state
        state.current_timeline = None;
        state.timelines.clear();
        state.media_pool.bins.clear();
        state.media_pool.clips.clear();
        state.color_state.current_clip = None;
        state.color_state.clip_grades.clear();
        state.timeline_items.items.clear();
        state.keyframe_state.timeline_item_keyframes.clear();
        state.render_state.render_queue.clear();
        state.render_state.active_renders.clear();

        Ok(serde_json::json!({
            "result": format!("Closed project '{}'", project_name),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_project_setting(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        let setting_name = args["setting_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("setting_name", "required string"))?;
        let setting_value = &args["setting_value"];

        Ok(serde_json::json!({
            "result": format!("Set project setting '{}' to {:?}", setting_name, setting_value),
            "operation_id": Uuid::new_v4().to_string(),
            "setting_name": setting_name,
            "setting_value": setting_value
        }))
    }

    // ---- Audio Transcription Operations ----
    async fn transcribe_audio(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        let language = args["language"].as_str().unwrap_or("en-US");

        // Simulate transcription processing
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(serde_json::json!({
            "result": format!("Started transcription for clip '{}' in language '{}'", clip_name, language),
            "transcription_id": Uuid::new_v4().to_string(),
            "clip_name": clip_name,
            "language": language,
            "estimated_duration": "45s",
            "status": "processing"
        }))
    }

    async fn clear_transcription(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;

        Ok(serde_json::json!({
            "result": format!("Cleared transcription for clip: {}", clip_name),
            "clip_name": clip_name,
            "status": "success"
        }))
    }

    // ---- NEW: Extended Project Management Operations ----
    async fn delete_media(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;

        // Remove clip from media pool
        state.media_pool.clips.remove(clip_name);

        Ok(serde_json::json!({
            "result": format!("Deleted media clip: {}", clip_name),
            "clip_name": clip_name,
            "status": "success"
        }))
    }

    async fn move_media_to_bin(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let bin_name = args["bin_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("bin_name", "parameter is required"))?;

        // Update clip's bin assignment
        if let Some(clip) = state.media_pool.clips.get_mut(clip_name) {
            clip.bin = Some(bin_name.to_string());
        }

        Ok(serde_json::json!({
            "result": format!("Moved clip '{}' to bin '{}'", clip_name, bin_name),
            "clip_name": clip_name,
            "bin_name": bin_name,
            "status": "success"
        }))
    }

    async fn export_folder(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let folder_name = args["folder_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("folder_name", "parameter is required")
        })?;
        let export_path = args["export_path"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("export_path", "parameter is required")
        })?;
        let export_type = args["export_type"].as_str().unwrap_or("DRB");

        Ok(serde_json::json!({
            "result": format!("Exported folder '{}' to '{}' as {}", folder_name, export_path, export_type),
            "folder_name": folder_name,
            "export_path": export_path,
            "export_type": export_type,
            "status": "success"
        }))
    }

    async fn transcribe_folder_audio(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let folder_name = args["folder_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("folder_name", "parameter is required")
        })?;
        let language = args["language"].as_str().unwrap_or("en-US");

        Ok(serde_json::json!({
            "result": format!("Started transcription for all clips in folder '{}' using language '{}'", folder_name, language),
            "folder_name": folder_name,
            "language": language,
            "status": "success"
        }))
    }

    async fn clear_folder_transcription(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let folder_name = args["folder_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("folder_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Cleared transcriptions for all clips in folder '{}'", folder_name),
            "folder_name": folder_name,
            "status": "success"
        }))
    }

    // ---- NEW: Cache and Optimization Operations ----
    async fn set_cache_mode(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let mode = args["mode"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("mode", "parameter is required"))?;

        if !["auto", "on", "off"].contains(&mode) {
            return Err(ResolveError::invalid_parameter(
                "mode",
                "mode must be 'auto', 'on', or 'off'",
            ));
        }

        Ok(serde_json::json!({
            "result": format!("Set cache mode to '{}'", mode),
            "mode": mode,
            "status": "success"
        }))
    }

    async fn set_optimized_media_mode(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let mode = args["mode"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("mode", "parameter is required"))?;

        if !["auto", "on", "off"].contains(&mode) {
            return Err(ResolveError::invalid_parameter(
                "mode",
                "mode must be 'auto', 'on', or 'off'",
            ));
        }

        Ok(serde_json::json!({
            "result": format!("Set optimized media mode to '{}'", mode),
            "mode": mode,
            "status": "success"
        }))
    }

    async fn set_proxy_mode(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let mode = args["mode"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("mode", "parameter is required"))?;

        if !["auto", "on", "off"].contains(&mode) {
            return Err(ResolveError::invalid_parameter(
                "mode",
                "mode must be 'auto', 'on', or 'off'",
            ));
        }

        Ok(serde_json::json!({
            "result": format!("Set proxy mode to '{}'", mode),
            "mode": mode,
            "status": "success"
        }))
    }

    async fn set_proxy_quality(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let quality = args["quality"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("quality", "parameter is required"))?;

        if !["quarter", "half", "threeQuarter", "full"].contains(&quality) {
            return Err(ResolveError::invalid_parameter(
                "mode",
                "quality must be 'quarter', 'half', 'threeQuarter', or 'full'",
            ));
        }

        Ok(serde_json::json!({
            "result": format!("Set proxy quality to '{}'", quality),
            "quality": quality,
            "status": "success"
        }))
    }

    async fn set_cache_path(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let path_type = args["path_type"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("path_type", "parameter is required"))?;
        let path = args["path"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("path", "parameter is required"))?;

        if !["local", "network"].contains(&path_type) {
            return Err(ResolveError::invalid_parameter(
                "mode",
                "path_type must be 'local' or 'network'",
            ));
        }

        Ok(serde_json::json!({
            "result": format!("Set {} cache path to '{}'", path_type, path),
            "path_type": path_type,
            "path": path,
            "status": "success"
        }))
    }

    async fn generate_optimized_media(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_names = args["clip_names"].as_array();

        let message = if let Some(clips) = clip_names {
            format!(
                "Started generating optimized media for {} clips",
                clips.len()
            )
        } else {
            "Started generating optimized media for all clips in media pool".to_string()
        };

        Ok(serde_json::json!({
            "result": message,
            "clip_names": clip_names,
            "status": "success"
        }))
    }

    async fn delete_optimized_media(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_names = args["clip_names"].as_array();

        let message = if let Some(clips) = clip_names {
            format!("Deleted optimized media for {} clips", clips.len())
        } else {
            "Deleted optimized media for all clips in media pool".to_string()
        };

        Ok(serde_json::json!({
            "result": message,
            "clip_names": clip_names,
            "status": "success"
        }))
    }

    // ---- NEW: Extended Color Operations ----
    async fn create_color_preset_album(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let album_name = args["album_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("album_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Created color preset album '{}'", album_name),
            "album_name": album_name,
            "status": "success"
        }))
    }

    async fn delete_color_preset_album(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let album_name = args["album_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("album_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Deleted color preset album '{}'", album_name),
            "album_name": album_name,
            "status": "success"
        }))
    }

    async fn export_all_power_grade_luts(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let export_dir = args["export_dir"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("export_dir", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Exported all PowerGrade LUTs to directory '{}'", export_dir),
            "export_dir": export_dir,
            "status": "success"
        }))
    }

    // ---- NEW: Layout and Interface Management ----
    async fn save_layout_preset(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("preset_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Saved layout preset '{}'", preset_name),
            "preset_name": preset_name,
            "status": "success"
        }))
    }

    async fn load_layout_preset(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("preset_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Loaded layout preset '{}'", preset_name),
            "preset_name": preset_name,
            "status": "success"
        }))
    }

    async fn export_layout_preset(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("preset_name", "parameter is required")
        })?;
        let export_path = args["export_path"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("export_path", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Exported layout preset '{}' to '{}'", preset_name, export_path),
            "preset_name": preset_name,
            "export_path": export_path,
            "status": "success"
        }))
    }

    async fn import_layout_preset(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let import_path = args["import_path"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("import_path", "parameter is required")
        })?;
        let preset_name = args["preset_name"].as_str();

        let name = preset_name.unwrap_or("Imported Layout");

        Ok(serde_json::json!({
            "result": format!("Imported layout preset from '{}' as '{}'", import_path, name),
            "import_path": import_path,
            "preset_name": name,
            "status": "success"
        }))
    }

    async fn delete_layout_preset(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("preset_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Deleted layout preset '{}'", preset_name),
            "preset_name": preset_name,
            "status": "success"
        }))
    }

    // ---- NEW: Application Control ----
    async fn quit_app(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let force = args["force"].as_bool().unwrap_or(false);
        let save_project = args["save_project"].as_bool().unwrap_or(true);

        let message = if force {
            "Force quitting DaVinci Resolve application"
        } else if save_project {
            "Saving project and quitting DaVinci Resolve application"
        } else {
            "Quitting DaVinci Resolve application without saving"
        };

        Ok(serde_json::json!({
            "result": message,
            "force": force,
            "save_project": save_project,
            "status": "success"
        }))
    }

    async fn restart_app(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let wait_seconds = args["wait_seconds"].as_i64().unwrap_or(5);

        Ok(serde_json::json!({
            "result": format!("Restarting DaVinci Resolve application (waiting {} seconds)", wait_seconds),
            "wait_seconds": wait_seconds,
            "status": "success"
        }))
    }

    async fn open_settings(&self, _state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
        Ok(serde_json::json!({
            "result": "Opened Project Settings dialog",
            "status": "success"
        }))
    }

    async fn open_app_preferences(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(serde_json::json!({
            "result": "Opened Application Preferences dialog",
            "status": "success"
        }))
    }

    // ---- NEW: Cloud Operations ----
    async fn create_cloud_project(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let project_name = args["project_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("project_name", "parameter is required")
        })?;
        let folder_path = args["folder_path"].as_str();

        let message = if let Some(path) = folder_path {
            format!(
                "Created cloud project '{}' in folder '{}'",
                project_name, path
            )
        } else {
            format!("Created cloud project '{}'", project_name)
        };

        Ok(serde_json::json!({
            "result": message,
            "project_name": project_name,
            "folder_path": folder_path,
            "status": "success"
        }))
    }

    async fn import_cloud_project(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let cloud_id = args["cloud_id"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("cloud_id", "parameter is required"))?;
        let project_name = args["project_name"].as_str();

        let message = if let Some(name) = project_name {
            format!("Imported cloud project '{}' as '{}'", cloud_id, name)
        } else {
            format!("Imported cloud project '{}'", cloud_id)
        };

        Ok(serde_json::json!({
            "result": message,
            "cloud_id": cloud_id,
            "project_name": project_name,
            "status": "success"
        }))
    }

    async fn restore_cloud_project(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let cloud_id = args["cloud_id"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("cloud_id", "parameter is required"))?;
        let project_name = args["project_name"].as_str();

        let message = if let Some(name) = project_name {
            format!("Restored cloud project '{}' as '{}'", cloud_id, name)
        } else {
            format!("Restored cloud project '{}'", cloud_id)
        };

        Ok(serde_json::json!({
            "result": message,
            "cloud_id": cloud_id,
            "project_name": project_name,
            "status": "success"
        }))
    }

    async fn export_project_to_cloud(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let project_name = args["project_name"].as_str().unwrap_or_else(|| {
            state
                .current_project
                .as_deref()
                .unwrap_or("Current Project")
        });

        Ok(serde_json::json!({
            "result": format!("Exported project '{}' to DaVinci Resolve cloud", project_name),
            "project_name": project_name,
            "status": "success"
        }))
    }

    async fn add_user_to_cloud_project(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let cloud_id = args["cloud_id"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("cloud_id", "parameter is required"))?;
        let user_email = args["user_email"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("user_email", "parameter is required")
        })?;
        let permissions = args["permissions"].as_str().unwrap_or("viewer");

        Ok(serde_json::json!({
            "result": format!("Added user '{}' to cloud project '{}' with '{}' permissions", user_email, cloud_id, permissions),
            "cloud_id": cloud_id,
            "user_email": user_email,
            "permissions": permissions,
            "status": "success"
        }))
    }

    async fn remove_user_from_cloud_project(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let cloud_id = args["cloud_id"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("cloud_id", "parameter is required"))?;
        let user_email = args["user_email"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("user_email", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Removed user '{}' from cloud project '{}'", user_email, cloud_id),
            "cloud_id": cloud_id,
            "user_email": user_email,
            "status": "success"
        }))
    }

    // ---- NEW: Object Inspection ----
    async fn object_help(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let object_type = args["object_type"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("object_type", "parameter is required")
        })?;

        let help_text = match object_type {
            "resolve" => "DaVinci Resolve main object - provides access to project manager and global settings",
            "project_manager" => "Project Manager - handles project creation, opening, and management",
            "project" => "Project object - contains timelines, media pool, and project settings",
            "media_pool" => "Media Pool - manages media clips, bins, and import/export operations",
            "timeline" => "Timeline object - handles timeline items, tracks, and editing operations",
            "media_storage" => "Media Storage - provides access to file system and media browsing",
            _ => "Unknown object type. Available types: resolve, project_manager, project, media_pool, timeline, media_storage"
        };

        Ok(serde_json::json!({
            "result": help_text,
            "object_type": object_type,
            "status": "success"
        }))
    }

    async fn inspect_custom_object(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let object_path = args["object_path"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("object_path", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Inspected object at path: {}", object_path),
            "object_path": object_path,
            "methods": ["GetName", "GetProperty", "SetProperty"],
            "properties": ["name", "type", "status"],
            "status": "success"
        }))
    }

    // ---- NEW: Project Properties ----
    async fn set_project_property(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let property_name = args["property_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("property_name", "parameter is required")
        })?;
        let property_value = &args["property_value"];

        Ok(serde_json::json!({
            "result": format!("Set project property '{}' to '{}'", property_name, property_value),
            "property_name": property_name,
            "property_value": property_value,
            "status": "success"
        }))
    }

    async fn set_timeline_format(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let width = args["width"]
            .as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("width", "parameter is required"))?;
        let height = args["height"]
            .as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("height", "parameter is required"))?;
        let frame_rate = args["frame_rate"].as_f64().ok_or_else(|| {
            ResolveError::invalid_parameter("frame_rate", "parameter is required")
        })?;
        let interlaced = args["interlaced"].as_bool().unwrap_or(false);

        Ok(serde_json::json!({
            "result": format!("Set timeline format to {}x{} @ {}fps{}", width, height, frame_rate, if interlaced { " (interlaced)" } else { "" }),
            "width": width,
            "height": height,
            "frame_rate": frame_rate,
            "interlaced": interlaced,
            "status": "success"
        }))
    }

    // ---- NEW: Timeline Object API ----
    async fn get_timeline_name(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();

        Ok(serde_json::json!({
            "result": format!("Timeline name: {}", timeline_name.unwrap_or("Current Timeline")),
            "timeline_name": timeline_name,
            "status": "success"
        }))
    }

    async fn set_timeline_name(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_name", "parameter is required")
        })?;
        let new_name = args["new_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("new_name", "parameter is required"))?;

        Ok(serde_json::json!({
            "result": format!("Renamed timeline '{}' to '{}'", timeline_name, new_name),
            "old_name": timeline_name,
            "new_name": new_name,
            "status": "success"
        }))
    }

    async fn get_timeline_frames(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();

        Ok(serde_json::json!({
            "result": "Timeline frame information retrieved",
            "timeline_name": timeline_name,
            "start_frame": 1001,
            "end_frame": 2000,
            "duration": 999,
            "status": "success"
        }))
    }

    async fn set_timeline_timecode(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let timecode = args["timecode"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timecode", "parameter is required"))?;

        Ok(serde_json::json!({
            "result": format!("Set timeline timecode to: {}", timecode),
            "timeline_name": timeline_name,
            "timecode": timecode,
            "status": "success"
        }))
    }

    async fn get_timeline_track_count(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let track_type = args["track_type"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("track_type", "parameter is required")
        })?;

        let count = match track_type {
            "video" => 4,
            "audio" => 8,
            "subtitle" => 2,
            _ => 0,
        };

        Ok(serde_json::json!({
            "result": format!("Track count for {}: {}", track_type, count),
            "timeline_name": timeline_name,
            "track_type": track_type,
            "count": count,
            "status": "success"
        }))
    }

    async fn get_timeline_items_in_track(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let track_type = args["track_type"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("track_type", "parameter is required")
        })?;
        let track_index = args["track_index"].as_i64().ok_or_else(|| {
            ResolveError::invalid_parameter("track_index", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Items in {} track {}", track_type, track_index),
            "timeline_name": timeline_name,
            "track_type": track_type,
            "track_index": track_index,
            "items": [
                {"id": "item_1", "name": "Clip 1", "start": 1001, "end": 1100},
                {"id": "item_2", "name": "Clip 2", "start": 1100, "end": 1200}
            ],
            "status": "success"
        }))
    }

    async fn add_timeline_marker(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let frame_id = args["frame_id"]
            .as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame_id", "parameter is required"))?;
        let color = args["color"].as_str().unwrap_or("Blue");
        let name = args["name"].as_str().unwrap_or("");
        let note = args["note"].as_str().unwrap_or("");

        Ok(serde_json::json!({
            "result": format!("Added timeline marker at frame {}", frame_id),
            "timeline_name": timeline_name,
            "frame_id": frame_id,
            "color": color,
            "name": name,
            "note": note,
            "status": "success"
        }))
    }

    async fn get_timeline_markers(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();

        Ok(serde_json::json!({
            "result": "Timeline markers retrieved",
            "timeline_name": timeline_name,
            "markers": [
                {"frame_id": 1050, "color": "Blue", "name": "Scene 1", "note": "Opening scene"},
                {"frame_id": 1200, "color": "Red", "name": "Cut", "note": "Hard cut here"}
            ],
            "status": "success"
        }))
    }

    async fn delete_timeline_marker(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let frame_num = args["frame_num"].as_f64();
        let color = args["color"].as_str();
        let custom_data = args["custom_data"].as_str();

        Ok(serde_json::json!({
            "result": "Timeline marker(s) deleted",
            "timeline_name": timeline_name,
            "frame_num": frame_num,
            "color": color,
            "custom_data": custom_data,
            "status": "success"
        }))
    }

    async fn duplicate_timeline(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let source_timeline_name = args["source_timeline_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("source_timeline_name", "parameter is required")
        })?;
        let new_timeline_name = args["new_timeline_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("new_timeline_name", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Duplicated timeline '{}' as '{}'", source_timeline_name, new_timeline_name),
            "source_timeline_name": source_timeline_name,
            "new_timeline_name": new_timeline_name,
            "status": "success"
        }))
    }

    async fn create_compound_clip(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let timeline_item_ids = args["timeline_item_ids"].as_array().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_ids", "parameter is required")
        })?;
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;

        Ok(serde_json::json!({
            "result": format!("Created compound clip '{}' from {} items", clip_name, timeline_item_ids.len()),
            "timeline_name": timeline_name,
            "clip_name": clip_name,
            "item_count": timeline_item_ids.len(),
            "status": "success"
        }))
    }

    async fn create_fusion_clip(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let timeline_item_ids = args["timeline_item_ids"].as_array().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_ids", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": format!("Created Fusion clip from {} items", timeline_item_ids.len()),
            "timeline_name": timeline_name,
            "item_count": timeline_item_ids.len(),
            "status": "success"
        }))
    }

    async fn export_timeline(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let file_name = args["file_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("file_name", "parameter is required"))?;
        let export_type = args["export_type"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("export_type", "parameter is required")
        })?;
        let export_subtype = args["export_subtype"].as_str();

        Ok(serde_json::json!({
            "result": format!("Exported timeline as {} to {}", export_type, file_name),
            "timeline_name": timeline_name,
            "file_name": file_name,
            "export_type": export_type,
            "export_subtype": export_subtype,
            "status": "success"
        }))
    }

    async fn insert_generator(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let generator_name = args["generator_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("generator_name", "parameter is required")
        })?;
        let generator_type = args["generator_type"].as_str().unwrap_or("standard");

        Ok(serde_json::json!({
            "result": format!("Inserted {} generator: {}", generator_type, generator_name),
            "timeline_name": timeline_name,
            "generator_name": generator_name,
            "generator_type": generator_type,
            "status": "success"
        }))
    }

    async fn insert_title(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let title_name = args["title_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("title_name", "parameter is required")
        })?;
        let title_type = args["title_type"].as_str().unwrap_or("standard");

        Ok(serde_json::json!({
            "result": format!("Inserted {} title: {}", title_type, title_name),
            "timeline_name": timeline_name,
            "title_name": title_name,
            "title_type": title_type,
            "status": "success"
        }))
    }

    async fn grab_still(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str();
        let still_frame_source = args["still_frame_source"].as_str();
        let grab_all = args["grab_all"].as_bool().unwrap_or(false);

        let action = if grab_all {
            "Grabbed all stills"
        } else {
            "Grabbed current still"
        };

        Ok(serde_json::json!({
            "result": action,
            "timeline_name": timeline_name,
            "still_frame_source": still_frame_source,
            "grab_all": grab_all,
            "status": "success"
        }))
    }

    // ---- NEW: TimelineItem Object API ----
    async fn get_timeline_item_property(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let property_key = args["property_key"].as_str();

        let properties = if let Some(key) = property_key {
            serde_json::json!({ key: "property_value" })
        } else {
            serde_json::json!({
                "name": "Timeline Item",
                "duration": 100,
                "start": 1001,
                "end": 1101,
                "left_offset": 0,
                "right_offset": 0
            })
        };

        Ok(serde_json::json!({
            "result": "Timeline item property retrieved",
            "timeline_item_id": timeline_item_id,
            "property_key": property_key,
            "properties": properties,
            "status": "success"
        }))
    }

    async fn set_timeline_item_property(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let property_key = args["property_key"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("property_key", "parameter is required")
        })?;
        let property_value = &args["property_value"];

        Ok(serde_json::json!({
            "result": format!("Set property '{}' on timeline item", property_key),
            "timeline_item_id": timeline_item_id,
            "property_key": property_key,
            "property_value": property_value,
            "status": "success"
        }))
    }

    async fn get_timeline_item_details(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": "Timeline item details retrieved",
            "timeline_item_id": timeline_item_id,
            "details": {
                "name": "Timeline Item",
                "duration": 100,
                "start": 1001,
                "end": 1101,
                "left_offset": 0,
                "right_offset": 0,
                "fusion_comp_count": 1,
                "num_nodes": 3,
                "takes_count": 1,
                "selected_take_index": 0
            },
            "status": "success"
        }))
    }

    async fn add_timeline_item_marker(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let frame_id = args["frame_id"]
            .as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame_id", "parameter is required"))?;
        let color = args["color"].as_str().unwrap_or("Blue");
        let name = args["name"].as_str().unwrap_or("");
        let note = args["note"].as_str().unwrap_or("");

        Ok(serde_json::json!({
            "result": format!("Added marker to timeline item at frame {}", frame_id),
            "timeline_item_id": timeline_item_id,
            "frame_id": frame_id,
            "color": color,
            "name": name,
            "note": note,
            "status": "success"
        }))
    }

    async fn get_timeline_item_markers(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;

        Ok(serde_json::json!({
            "result": "Timeline item markers retrieved",
            "timeline_item_id": timeline_item_id,
            "markers": [
                {"frame_id": 10, "color": "Blue", "name": "Start", "note": "Beginning of clip"},
                {"frame_id": 50, "color": "Red", "name": "Mid", "note": "Middle point"}
            ],
            "status": "success"
        }))
    }

    async fn delete_timeline_item_marker(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let frame_num = args["frame_num"].as_f64();
        let color = args["color"].as_str();
        let custom_data = args["custom_data"].as_str();

        Ok(serde_json::json!({
            "result": "Timeline item marker(s) deleted",
            "timeline_item_id": timeline_item_id,
            "frame_num": frame_num,
            "color": color,
            "custom_data": custom_data,
            "status": "success"
        }))
    }

    async fn timeline_item_flag(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let color = args["color"].as_str();

        let action = if color.is_some() {
            format!("Added {} flag to timeline item", color.unwrap())
        } else {
            "Retrieved flags from timeline item".to_string()
        };

        Ok(serde_json::json!({
            "result": action,
            "timeline_item_id": timeline_item_id,
            "color": color,
            "flags": ["Red", "Blue"],
            "status": "success"
        }))
    }

    async fn timeline_item_color(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let color_name = args["color_name"].as_str();

        let action = if let Some(color) = color_name {
            format!("Set timeline item color to {}", color)
        } else {
            "Retrieved timeline item color".to_string()
        };

        Ok(serde_json::json!({
            "result": action,
            "timeline_item_id": timeline_item_id,
            "color_name": color_name.unwrap_or("Orange"),
            "status": "success"
        }))
    }

    async fn fusion_comp(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let comp_index = args["comp_index"].as_i64();
        let comp_name = args["comp_name"].as_str();
        let file_path = args["file_path"].as_str();

        Ok(serde_json::json!({
            "result": "Fusion composition operation completed",
            "timeline_item_id": timeline_item_id,
            "comp_index": comp_index,
            "comp_name": comp_name,
            "file_path": file_path,
            "status": "success"
        }))
    }

    async fn version(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let version_name = args["version_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("version_name", "parameter is required")
        })?;
        let version_type = args["version_type"].as_str().unwrap_or("local");

        Ok(serde_json::json!({
            "result": format!("Version operation completed for '{}'", version_name),
            "timeline_item_id": timeline_item_id,
            "version_name": version_name,
            "version_type": version_type,
            "status": "success"
        }))
    }

    async fn stereo_params(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let params = &args["params"];

        Ok(serde_json::json!({
            "result": "Stereo parameters operation completed",
            "timeline_item_id": timeline_item_id,
            "params": params,
            "status": "success"
        }))
    }

    async fn node_lut(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let node_index = args["node_index"].as_i64().ok_or_else(|| {
            ResolveError::invalid_parameter("node_index", "parameter is required")
        })?;
        let lut_path = args["lut_path"].as_str();

        let action = if lut_path.is_some() {
            format!("Set LUT on node {} to {}", node_index, lut_path.unwrap())
        } else {
            format!("Retrieved LUT from node {}", node_index)
        };

        Ok(serde_json::json!({
            "result": action,
            "timeline_item_id": timeline_item_id,
            "node_index": node_index,
            "lut_path": lut_path,
            "status": "success"
        }))
    }

    async fn set_cdl(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let cdl_map = &args["cdl_map"];

        Ok(serde_json::json!({
            "result": "CDL parameters set on timeline item",
            "timeline_item_id": timeline_item_id,
            "cdl_map": cdl_map,
            "status": "success"
        }))
    }

    async fn take(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_item_id", "parameter is required")
        })?;
        let media_pool_item = args["media_pool_item"].as_str();
        let take_index = args["take_index"].as_i64();

        Ok(serde_json::json!({
            "result": "Take operation completed",
            "timeline_item_id": timeline_item_id,
            "media_pool_item": media_pool_item,
            "take_index": take_index,
            "status": "success"
        }))
    }

    async fn copy_grades(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let source_timeline_item_id =
            args["source_timeline_item_id"].as_str().ok_or_else(|| {
                ResolveError::invalid_parameter("source_timeline_item_id", "parameter is required")
            })?;
        let target_timeline_item_ids =
            args["target_timeline_item_ids"].as_array().ok_or_else(|| {
                ResolveError::invalid_parameter("target_timeline_item_ids", "parameter is required")
            })?;

        Ok(serde_json::json!({
            "result": format!("Copied grades from {} to {} items", source_timeline_item_id, target_timeline_item_ids.len()),
            "source_timeline_item_id": source_timeline_item_id,
            "target_count": target_timeline_item_ids.len(),
            "status": "success"
        }))
    }

    // ---- MediaPoolItem Object API Implementation ----

    async fn get_media_pool_item_list(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let clips: Vec<Value> = state
            .media_pool
            .clips
            .iter()
            .map(|(name, clip)| {
                json!({
                    "name": name,
                    "file_path": clip.file_path,
                    "bin": clip.bin,
                    "linked": clip.linked,
                    "proxy_path": clip.proxy_path
                })
            })
            .collect();

        Ok(json!({
            "success": true,
            "clips": clips,
            "count": clips.len(),
            "operation_id": format!("get_media_pool_item_list_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_media_pool_item_name(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");

        if let Some(clip) = state.media_pool.clips.get(clip_name) {
            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "display_name": clip.name,
                "operation_id": format!("get_media_pool_item_name_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("get_media_pool_item_name_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_media_pool_item_property(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");
        let property_name = args["property_name"].as_str().unwrap_or("File Name");

        if let Some(clip) = state.media_pool.clips.get(clip_name) {
            let property_value = match property_name {
                "File Name" => clip.file_path.clone(),
                "Clip Name" => clip.name.clone(),
                "Bin" => clip.bin.clone().unwrap_or_else(|| "Master".to_string()),
                "Linked" => clip.linked.to_string(),
                "Proxy Path" => clip
                    .proxy_path
                    .clone()
                    .unwrap_or_else(|| "None".to_string()),
                _ => format!("Property '{}' not available", property_name),
            };

            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "property_name": property_name,
                "property_value": property_value,
                "operation_id": format!("get_media_pool_item_property_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("get_media_pool_item_property_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn set_media_pool_item_property(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");
        let property_name = args["property_name"].as_str().unwrap_or("Clip Name");
        let property_value = args["property_value"].as_str().unwrap_or("");

        if let Some(clip) = state.media_pool.clips.get_mut(clip_name) {
            match property_name {
                "Clip Name" => clip.name = property_value.to_string(),
                "Bin" => clip.bin = Some(property_value.to_string()),
                "Proxy Path" => clip.proxy_path = Some(property_value.to_string()),
                _ => {
                    return Ok(json!({
                        "success": false,
                        "error": format!("Property '{}' is read-only or not supported", property_name),
                        "operation_id": format!("set_media_pool_item_property_{}", chrono::Utc::now().timestamp())
                    }));
                }
            }

            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "property_name": property_name,
                "property_value": property_value,
                "message": format!("Set property '{}' to '{}' for clip '{}'", property_name, property_value, clip_name),
                "operation_id": format!("set_media_pool_item_property_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("set_media_pool_item_property_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_media_pool_item_metadata(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");
        let metadata_type = args["metadata_type"].as_str().unwrap_or("File Name");

        if let Some(clip) = state.media_pool.clips.get(clip_name) {
            let metadata_value = match metadata_type {
                "File Name" => clip.file_path.clone(),
                "Clip Name" => clip.name.clone(),
                "Duration" => "00:00:10:00".to_string(), // Simulated duration
                "Frame Rate" => "24".to_string(),
                "Resolution" => "1920x1080".to_string(),
                "Codec" => "H.264".to_string(),
                "Date Created" => chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                _ => format!("Metadata '{}' not available", metadata_type),
            };

            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "metadata_type": metadata_type,
                "metadata_value": metadata_value,
                "operation_id": format!("get_media_pool_item_metadata_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("get_media_pool_item_metadata_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn set_media_pool_item_metadata(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");
        let metadata_type = args["metadata_type"].as_str().unwrap_or("Clip Name");
        let metadata_value = args["metadata_value"].as_str().unwrap_or("");

        if state.media_pool.clips.contains_key(clip_name) {
            // In simulation mode, we just acknowledge the metadata change
            // In real mode, this would actually modify the clip metadata
            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "metadata_type": metadata_type,
                "metadata_value": metadata_value,
                "message": format!("Set metadata '{}' to '{}' for clip '{}'", metadata_type, metadata_value, clip_name),
                "operation_id": format!("set_media_pool_item_metadata_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("set_media_pool_item_metadata_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_media_pool_item_markers(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");

        if state.media_pool.clips.contains_key(clip_name) {
            // Simulate some markers for the clip
            let markers = vec![
                json!({
                    "frame": 24,
                    "color": "Red",
                    "note": "Important scene",
                    "duration": 1
                }),
                json!({
                    "frame": 120,
                    "color": "Blue",
                    "note": "Cut point",
                    "duration": 1
                }),
            ];

            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "markers": markers,
                "count": markers.len(),
                "operation_id": format!("get_media_pool_item_markers_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("get_media_pool_item_markers_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_media_pool_item_flag_list(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");

        if state.media_pool.clips.contains_key(clip_name) {
            // Simulate flag list for the clip
            let flags = vec![
                "Blue", "Cyan", "Green", "Yellow", "Red", "Pink", "Purple", "Fuchsia", "Rose",
                "Lavender", "Sky", "Mint", "Lemon", "Sand", "Cocoa", "Cream",
            ];

            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "flags": flags,
                "current_flag": "None",
                "operation_id": format!("get_media_pool_item_flag_list_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("get_media_pool_item_flag_list_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_media_pool_item_clip_color(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str().unwrap_or("default_clip");

        if state.media_pool.clips.contains_key(clip_name) {
            Ok(json!({
                "success": true,
                "clip_name": clip_name,
                "clip_color": "Orange", // Default simulated color
                "operation_id": format!("get_media_pool_item_clip_color_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("get_media_pool_item_clip_color_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn set_media_pool_item_name(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let new_name = args["new_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("new_name", "parameter is required"))?;

        if let Some(clip) = state.media_pool.clips.get_mut(clip_name) {
            clip.name = new_name.to_string();
            Ok(json!({
                "success": true,
                "result": format!("Renamed clip from '{}' to '{}'", clip_name, new_name),
                "old_name": clip_name,
                "new_name": new_name,
                "operation_id": format!("set_media_pool_item_name_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Clip '{}' not found in media pool", clip_name),
                "operation_id": format!("set_media_pool_item_name_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn add_media_pool_item_marker(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let frame_id = args["frame_id"].as_i64().unwrap_or(0);
        let color = args["color"].as_str().unwrap_or("Red");
        let name = args["name"].as_str().unwrap_or("");
        let note = args["note"].as_str().unwrap_or("");

        Ok(json!({
            "success": true,
            "result": format!("Added marker '{}' at frame {} for clip '{}'", name, frame_id, clip_name),
            "clip_name": clip_name,
            "frame_id": frame_id,
            "color": color,
            "name": name,
            "note": note,
            "operation_id": format!("add_media_pool_item_marker_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn add_media_pool_item_flag(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let color = args["color"].as_str().unwrap_or("Blue");

        Ok(json!({
            "success": true,
            "result": format!("Added {} flag to clip '{}'", color, clip_name),
            "clip_name": clip_name,
            "color": color,
            "operation_id": format!("add_media_pool_item_flag_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn set_media_pool_item_clip_color(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let color_name = args["color_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("color_name", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Set clip color to {} for clip '{}'", color_name, clip_name),
            "clip_name": clip_name,
            "color_name": color_name,
            "operation_id": format!("set_media_pool_item_clip_color_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn link_media_pool_item_proxy_media(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let proxy_media_file_path = args["proxy_media_file_path"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("proxy_media_file_path", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Linked proxy media '{}' to clip '{}'", proxy_media_file_path, clip_name),
            "clip_name": clip_name,
            "proxy_media_file_path": proxy_media_file_path,
            "operation_id": format!("link_media_pool_item_proxy_media_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn unlink_media_pool_item_proxy_media(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;

        Ok(json!({
            "success": true,
            "result": format!("Unlinked proxy media from clip '{}'", clip_name),
            "clip_name": clip_name,
            "operation_id": format!("unlink_media_pool_item_proxy_media_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn transcribe_media_pool_item_audio(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;
        let language = args["language"].as_str().unwrap_or("en-US");

        Ok(json!({
            "success": true,
            "result": format!("Started transcription for clip '{}' in language '{}'", clip_name, language),
            "clip_name": clip_name,
            "language": language,
            "operation_id": format!("transcribe_media_pool_item_audio_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn clear_media_pool_item_transcription(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_name = args["clip_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "parameter is required"))?;

        Ok(json!({
            "success": true,
            "result": format!("Cleared transcription for clip '{}'", clip_name),
            "clip_name": clip_name,
            "operation_id": format!("clear_media_pool_item_transcription_{}", chrono::Utc::now().timestamp())
        }))
    }

    // ---- NEW: Missing API Method Implementations ----

    async fn get_fusion_tool_list(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let selected_only = args["selected_only"].as_bool().unwrap_or(false);
        let tool_type = args["tool_type"].as_str();

        let tools = if selected_only {
            vec!["Transform", "Merge", "ColorCorrector"]
        } else {
            vec![
                "Transform",
                "Merge",
                "ColorCorrector",
                "Blur",
                "Glow",
                "Sharpen",
                "MediaIn",
                "MediaOut",
            ]
        };

        let filtered_tools = if let Some(filter_type) = tool_type {
            tools
                .into_iter()
                .filter(|&tool| tool.contains(filter_type))
                .collect()
        } else {
            tools
        };

        Ok(json!({
            "success": true,
            "result": "Retrieved Fusion tool list",
            "tools": filtered_tools,
            "count": filtered_tools.len(),
            "selected_only": selected_only,
            "tool_type": tool_type,
            "operation_id": format!("get_fusion_tool_list_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_audio_track_count(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved audio track count",
            "track_count": 8,
            "operation_id": format!("get_audio_track_count_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_project_timeline_count(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let count = state.timelines.len();
        Ok(json!({
            "success": true,
            "result": "Retrieved project timeline count",
            "timeline_count": count,
            "operation_id": format!("get_project_timeline_count_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_gallery_still_albums(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let albums = vec!["PowerGrade", "Stills", "LUTs", "Custom"];
        Ok(json!({
            "success": true,
            "result": "Retrieved gallery still albums",
            "albums": albums,
            "count": albums.len(),
            "operation_id": format!("get_gallery_still_albums_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_media_pool_root_folder(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved media pool root folder",
            "folder_name": "Master",
            "folder_id": "root_folder_001",
            "operation_id": format!("get_media_pool_root_folder_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn add_fusion_tool(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let tool_name = args["tool_name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("tool_name", "parameter is required"))?;
        let x = args["x"].as_f64().unwrap_or(0.0);
        let y = args["y"].as_f64().unwrap_or(0.0);

        Ok(json!({
            "success": true,
            "result": format!("Added Fusion tool '{}' at position ({}, {})", tool_name, x, y),
            "tool_name": tool_name,
            "position": {"x": x, "y": y},
            "tool_id": format!("tool_{}", chrono::Utc::now().timestamp()),
            "operation_id": format!("add_fusion_tool_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_audio_track_name(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let track_index = args["track_index"].as_i64().ok_or_else(|| {
            ResolveError::invalid_parameter("track_index", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Retrieved audio track name for track {}", track_index),
            "track_index": track_index,
            "track_name": format!("Audio Track {}", track_index),
            "operation_id": format!("get_audio_track_name_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn set_audio_track_name(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let track_index = args["track_index"].as_i64().ok_or_else(|| {
            ResolveError::invalid_parameter("track_index", "parameter is required")
        })?;
        let track_name = args["track_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("track_name", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Set audio track {} name to '{}'", track_index, track_name),
            "track_index": track_index,
            "track_name": track_name,
            "operation_id": format!("set_audio_track_name_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn add_gallery_still_album(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let album_name = args["album_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("album_name", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Added gallery still album '{}'", album_name),
            "album_name": album_name,
            "album_id": format!("album_{}", chrono::Utc::now().timestamp()),
            "operation_id": format!("add_gallery_still_album_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn add_media_pool_sub_folder(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let name = args["name"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "parameter is required"))?;
        let _parent_folder = args["parent_folder"].as_str();

        // Check if bin already exists - if so, return success (idempotent operation)
        if state.media_pool.bins.contains_key(name) {
            return Ok(json!({
                "success": true,
                "result": format!("Media pool sub folder '{}' already exists", name),
                "folder_name": name,
                "folder_id": format!("folder_{}", chrono::Utc::now().timestamp()),
                "operation_id": format!("add_media_pool_sub_folder_{}", chrono::Utc::now().timestamp()),
                "already_existed": true
            }));
        }

        let bin = Bin {
            name: name.to_string(),
            clips: Vec::new(),
        };

        state.media_pool.bins.insert(name.to_string(), bin);

        Ok(json!({
            "success": true,
            "result": format!("Added media pool sub folder '{}'", name),
            "folder_name": name,
            "folder_id": format!("folder_{}", chrono::Utc::now().timestamp()),
            "operation_id": format!("add_media_pool_sub_folder_{}", chrono::Utc::now().timestamp()),
            "already_existed": false
        }))
    }

    async fn append_to_timeline(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let clip_info = args["clip_info"]
            .as_array()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_info", "parameter is required"))?;
        let timeline_name = args["timeline_name"].as_str();

        let clip_names: Vec<String> = clip_info
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        Ok(json!({
            "success": true,
            "result": format!("Appended {} clips to timeline", clip_names.len()),
            "clips": clip_names,
            "timeline_name": timeline_name,
            "operation_id": format!("append_to_timeline_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_project_timeline_by_index(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_index = args["timeline_index"].as_i64().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_index", "parameter is required")
        })?;

        let timeline_names: Vec<&String> = state.timelines.keys().collect();
        let index = (timeline_index - 1) as usize; // Convert to 0-based index

        if index < timeline_names.len() {
            let timeline_name = timeline_names[index];
            Ok(json!({
                "success": true,
                "result": format!("Retrieved timeline at index {}", timeline_index),
                "timeline_index": timeline_index,
                "timeline_name": timeline_name,
                "operation_id": format!("get_project_timeline_by_index_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Timeline index {} out of range", timeline_index),
                "operation_id": format!("get_project_timeline_by_index_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_project_current_timeline(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved current timeline",
            "current_timeline": state.current_timeline,
            "operation_id": format!("get_project_current_timeline_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn set_project_current_timeline(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let timeline_name = args["timeline_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("timeline_name", "parameter is required")
        })?;

        if state.timelines.contains_key(timeline_name) {
            state.current_timeline = Some(timeline_name.to_string());
            Ok(json!({
                "success": true,
                "result": format!("Set current timeline to '{}'", timeline_name),
                "timeline_name": timeline_name,
                "operation_id": format!("set_project_current_timeline_{}", chrono::Utc::now().timestamp())
            }))
        } else {
            Ok(json!({
                "success": false,
                "error": format!("Timeline '{}' not found", timeline_name),
                "operation_id": format!("set_project_current_timeline_{}", chrono::Utc::now().timestamp())
            }))
        }
    }

    async fn get_project_name(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved project name",
            "project_name": state.current_project,
            "operation_id": format!("get_project_name_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn set_project_name(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let project_name = args["project_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("project_name", "parameter is required")
        })?;

        state.current_project = Some(project_name.to_string());
        Ok(json!({
            "success": true,
            "result": format!("Set project name to '{}'", project_name),
            "project_name": project_name,
            "operation_id": format!("set_project_name_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_project_unique_id(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved project unique ID",
            "unique_id": format!("project_{}", chrono::Utc::now().timestamp()),
            "operation_id": format!("get_project_unique_id_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_project_render_job_list(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let job_list: Vec<&RenderJob> = state.render_state.render_queue.iter().collect();
        Ok(json!({
            "success": true,
            "result": "Retrieved project render job list",
            "job_count": job_list.len(),
            "jobs": job_list.iter().map(|job| json!({
                "id": job.id,
                "timeline_name": job.timeline_name,
                "preset_name": job.preset_name,
                "status": format!("{:?}", job.status)
            })).collect::<Vec<_>>(),
            "operation_id": format!("get_project_render_job_list_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn start_project_rendering(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let _job_ids = args["job_ids"].as_array();
        let _is_interactive_mode = args["is_interactive_mode"].as_bool().unwrap_or(false);

        // Start rendering queued jobs
        for job in &mut state.render_state.render_queue {
            if matches!(job.status, RenderJobStatus::Queued) {
                job.status = RenderJobStatus::Rendering;
            }
        }

        Ok(json!({
            "success": true,
            "result": "Started project rendering",
            "operation_id": format!("start_project_rendering_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn stop_project_rendering(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        // Stop all rendering jobs
        for job in &mut state.render_state.render_queue {
            if matches!(job.status, RenderJobStatus::Rendering) {
                job.status = RenderJobStatus::Queued;
            }
        }

        Ok(json!({
            "success": true,
            "result": "Stopped project rendering",
            "operation_id": format!("stop_project_rendering_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn is_project_rendering_in_progress(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let is_rendering = state
            .render_state
            .render_queue
            .iter()
            .any(|job| matches!(job.status, RenderJobStatus::Rendering));

        Ok(json!({
            "success": true,
            "result": "Checked project rendering status",
            "is_rendering": is_rendering,
            "operation_id": format!("is_project_rendering_in_progress_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_project_preset_list(
        &self,
        state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let preset_names: Vec<&String> = state.render_state.render_presets.keys().collect();
        Ok(json!({
            "success": true,
            "result": "Retrieved project preset list",
            "presets": preset_names,
            "count": preset_names.len(),
            "operation_id": format!("get_project_preset_list_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn load_project_render_preset(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("preset_name", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Loaded render preset '{}'", preset_name),
            "preset_name": preset_name,
            "operation_id": format!("load_project_render_preset_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn save_as_new_project_render_preset(
        &self,
        state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("preset_name", "parameter is required")
        })?;

        let preset = RenderPreset {
            name: preset_name.to_string(),
            format: "MP4".to_string(),
            codec: "H.264".to_string(),
            resolution: (1920, 1080),
            frame_rate: 24.0,
            quality: RenderQuality::High,
            audio_codec: "AAC".to_string(),
            audio_bitrate: 320,
            created_at: chrono::Utc::now(),
        };

        state
            .render_state
            .render_presets
            .insert(preset_name.to_string(), preset);

        Ok(json!({
            "success": true,
            "result": format!("Saved new render preset '{}'", preset_name),
            "preset_name": preset_name,
            "operation_id": format!("save_as_new_project_render_preset_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_current_project_render_format_and_codec(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved current render format and codec",
            "format": "QuickTime",
            "codec": "H.264",
            "operation_id": format!("get_current_project_render_format_and_codec_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn set_current_project_render_format_and_codec(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let format = args["format"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("format", "parameter is required"))?;
        let codec = args["codec"]
            .as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("codec", "parameter is required"))?;

        Ok(json!({
            "success": true,
            "result": format!("Set render format to '{}' and codec to '{}'", format, codec),
            "format": format,
            "codec": codec,
            "operation_id": format!("set_current_project_render_format_and_codec_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_current_project_render_mode(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        Ok(json!({
            "success": true,
            "result": "Retrieved current render mode",
            "render_mode": "Single clip",
            "operation_id": format!("get_current_project_render_mode_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn set_current_project_render_mode(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let render_mode = args["render_mode"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("render_mode", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Set render mode to '{}'", render_mode),
            "render_mode": render_mode,
            "operation_id": format!("set_current_project_render_mode_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn get_project_color_groups_list(
        &self,
        _state: &mut ResolveState,
        _args: Value,
    ) -> ResolveResult<Value> {
        let color_groups = vec!["Group 1", "Group 2", "Group 3"];
        Ok(json!({
            "success": true,
            "result": "Retrieved project color groups list",
            "color_groups": color_groups,
            "count": color_groups.len(),
            "operation_id": format!("get_project_color_groups_list_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn add_project_color_group(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let group_name = args["group_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("group_name", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Added project color group '{}'", group_name),
            "group_name": group_name,
            "operation_id": format!("add_project_color_group_{}", chrono::Utc::now().timestamp())
        }))
    }

    async fn delete_project_color_group(
        &self,
        _state: &mut ResolveState,
        args: Value,
    ) -> ResolveResult<Value> {
        let group_name = args["group_name"].as_str().ok_or_else(|| {
            ResolveError::invalid_parameter("group_name", "parameter is required")
        })?;

        Ok(json!({
            "success": true,
            "result": format!("Deleted project color group '{}'", group_name),
            "group_name": group_name,
            "operation_id": format!("delete_project_color_group_{}", chrono::Utc::now().timestamp())
        }))
    }
}

impl ResolveState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn initialize(&mut self) -> ResolveResult<()> {
        // Initialize connection to DaVinci Resolve
        self.operation_count += 1;
        Ok(())
    }

    pub async fn switch_page(&mut self, page: &str) -> ResolveResult<String> {
        self.current_page = page.to_string();
        self.operation_count += 1;
        Ok(format!("Switched to {} page", page))
    }

    pub async fn create_empty_timeline(&mut self, args: Value) -> ResolveResult<String> {
        let name = args["name"].as_str().unwrap_or("New Timeline").to_string();
        let frame_rate = args["frame_rate"].as_str().map(|s| s.to_string());
        let resolution_width = args["resolution_width"].as_i64().map(|i| i as i32);
        let resolution_height = args["resolution_height"].as_i64().map(|i| i as i32);

        let timeline = Timeline {
            name: name.clone(),
            frame_rate,
            resolution_width,
            resolution_height,
            markers: Vec::new(),
        };

        self.timelines.insert(name.clone(), timeline);
        self.current_timeline = Some(name.clone());
        self.operation_count += 1;

        Ok(format!("Created timeline: {}", name))
    }

    pub async fn add_marker(&mut self, args: Value) -> ResolveResult<String> {
        let frame = args["frame"].as_i64().map(|i| i as i32);
        let color = args["color"].as_str().unwrap_or("Blue").to_string();
        let note = args["note"].as_str().unwrap_or("").to_string();

        let marker = Marker {
            frame,
            color: color.clone(),
            note: note.clone(),
        };

        if let Some(timeline_name) = &self.current_timeline {
            if let Some(timeline) = self.timelines.get_mut(timeline_name) {
                timeline.markers.push(marker);
                self.operation_count += 1;
                return Ok(format!("Added {} marker: {}", color, note));
            }
        }

        Err(ResolveError::internal("No current timeline"))
    }

    pub async fn list_timelines(&mut self) -> ResolveResult<String> {
        let timeline_names: Vec<String> = self.timelines.keys().cloned().collect();
        self.operation_count += 1;
        Ok(format!("Timelines: {:?}", timeline_names))
    }
}

impl Default for ResolveBridge {
    fn default() -> Self {
        Self::new(ConnectionMode::Simulation)
    }
}

// Re-export for convenience
pub use self::ResolveBridge as Bridge;
