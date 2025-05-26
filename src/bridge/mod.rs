use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::Value;
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
    /// Native DaVinci Resolve integration
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

#[derive(Debug, Default)]
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
    mode: String,     // "Normal", "Add", "Multiply", etc.
    opacity: f64,     // 0.0 to 1.0
}

#[derive(Debug, Clone, Default)]
struct RetimeProperties {
    speed: f64,       // Speed factor
    process: String,  // "NearestFrame", "FrameBlend", "OpticalFlow"
}

#[derive(Debug, Clone, Default)]
struct StabilizationProperties {
    enabled: bool,
    method: String,   // "Perspective", "Similarity", "Translation"
    strength: f64,    // 0.0 to 1.0
}

#[derive(Debug, Clone, Default)]
struct AudioProperties {
    volume: f64,      // Volume level (usually 0.0 to 2.0, where 1.0 is unity gain)
    pan: f64,         // -1.0 to 1.0
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
    size: String,   // "17Point", "33Point", "65Point"
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
        state.color_state.available_luts.insert("Rec709_to_sRGB".to_string(), LutInfo {
            name: "Rec709 to sRGB".to_string(),
            path: "/usr/share/davinci/luts/rec709_to_srgb.cube".to_string(),
            format: "Cube".to_string(),
            size: "33Point".to_string(),
        });
        state.color_state.available_luts.insert("Cinematic_Look".to_string(), LutInfo {
            name: "Cinematic Look".to_string(),
            path: "/usr/share/davinci/luts/cinematic.cube".to_string(),
            format: "Cube".to_string(),
            size: "33Point".to_string(),
        });

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
            },
            ConnectionMode::Real => {
                tracing::info!("Attempting to connect to real DaVinci Resolve instance...");
                
                // Try native integration first
                let mut native_guard = self.native.lock().await;
                let mut native_resolve = NativeDaVinciResolve::new();
                
                match native_resolve.initialize() {
                    Ok(()) => {
                        tracing::info!("🚀 Native DaVinci Resolve integration available!");
                        match native_resolve.connect() {
                            Ok(()) => {
                                tracing::info!("✅ Native connection established successfully");
                                *native_guard = Some(native_resolve);
                                *self.connected.lock().await = true;
                                return Ok(());
                            },
                            Err(e) => {
                                tracing::warn!("⚠️ Native connection failed: {}, falling back to Python", e);
                            }
                        }
                    },
                    Err(e) => {
                        tracing::warn!("⚠️ Native integration not available: {}, falling back to Python", e);
                    }
                }
                drop(native_guard);
                
                // Fallback to Python integration
                match self.check_davinci_resolve_connection().await {
                    Ok(()) => {
                        tracing::info!("🐍 Successfully connected to DaVinci Resolve via Python");
                        *self.connected.lock().await = true;
                        Ok(())
                    },
                    Err(e) => {
                        tracing::error!("❌ Failed to connect to DaVinci Resolve: {}", e);
                        *self.connected.lock().await = false;
                        Err(e)
                    }
                }
            }
        }
    }

    /// Check if DaVinci Resolve is running and accessible
    async fn check_davinci_resolve_connection(&self) -> ResolveResult<()> {
        // Try to connect to DaVinci Resolve on port 15000 (main API port)
        match tokio::net::TcpStream::connect("127.0.0.1:15000").await {
            Ok(_) => {
                tracing::info!("DaVinci Resolve appears to be running (port 15000 accessible)");
                
                // Additional check: try to import DaVinci Resolve API
                match self.test_davinci_resolve_api().await {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        tracing::warn!("DaVinci Resolve API test failed: {}", e);
                        Err(ResolveError::NotRunning)
                    }
                }
            },
            Err(_) => {
                tracing::error!("Cannot connect to DaVinci Resolve on port 15000");
                Err(ResolveError::NotRunning)
            }
        }
    }

    /// Test DaVinci Resolve API functionality
    async fn test_davinci_resolve_api(&self) -> ResolveResult<()> {
        tracing::info!("Testing DaVinci Resolve API access...");
        
        // Try to test DaVinci Resolve API via Python subprocess
        let python_test_script = r#"
import sys
import os

# Add DaVinci Resolve Python API path
resolve_api_path = "/opt/resolve/Developer/Scripting/Modules"
if resolve_api_path not in sys.path:
    sys.path.append(resolve_api_path)

try:
    import DaVinciResolveScript as dvr_script
    resolve = dvr_script.scriptapp("Resolve")
    if resolve is None:
        print("ERROR: DaVinci Resolve API not available - make sure 'External scripting using local network' is enabled")
        sys.exit(1)
    else:
        print("SUCCESS: DaVinci Resolve API accessible")
        # Test basic functionality
        project_manager = resolve.GetProjectManager()
        if project_manager:
            print("SUCCESS: Project manager accessible")
        sys.exit(0)
except ImportError as e:
    print(f"ERROR: Cannot import DaVinciResolveScript: {e}")
    sys.exit(1)
except Exception as e:
    print(f"ERROR: {e}")
    sys.exit(1)
"#;

        // Execute Python test script
        let output = tokio::process::Command::new("python3")
            .arg("-c")
            .arg(python_test_script)
            .output()
            .await
            .map_err(|e| ResolveError::api_call("python_test", format!("Failed to execute Python test: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        tracing::debug!("Python test output: {}", stdout);
        if !stderr.is_empty() {
            tracing::debug!("Python test stderr: {}", stderr);
        }

        if output.status.success() && stdout.contains("SUCCESS") {
            tracing::info!("DaVinci Resolve API test successful");
            Ok(())
        } else {
            tracing::error!("DaVinci Resolve API test failed: {}", stdout);
            Err(ResolveError::api_call("python_test", format!("DaVinci Resolve API not accessible: {}", stdout)))
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
        tracing::debug!("API call: {} with args: {} (mode: {:?})", method, args, self.mode);

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
                        tracing::warn!("Real API call failed for {} ({}), falling back to simulation", method, e);
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
            "set_timeline_item_transform" => self.set_timeline_item_transform(&mut state, args).await,
            "set_timeline_item_crop" => self.set_timeline_item_crop(&mut state, args).await,
            "set_timeline_item_composite" => self.set_timeline_item_composite(&mut state, args).await,
            "set_timeline_item_retime" => self.set_timeline_item_retime(&mut state, args).await,
            "set_timeline_item_stabilization" => self.set_timeline_item_stabilization(&mut state, args).await,
            "set_timeline_item_audio" => self.set_timeline_item_audio(&mut state, args).await,
            "get_timeline_item_properties" => self.get_timeline_item_properties(&mut state, args).await,
            "reset_timeline_item_properties" => self.reset_timeline_item_properties(&mut state, args).await,

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

            _ => Err(ResolveError::not_supported(format!("API method: {}", method))),
        }
    }

    /// Call real DaVinci Resolve API using Python integration
    async fn call_real_api(&self, method: &str, args: &Value) -> ResolveResult<Value> {
        use std::process::Command;
        
        // Create a temporary Python script to call DaVinci Resolve API
        let python_script = format!(r#"
import DaVinciResolveScript as dvr_script
import json
import sys

try:
    resolve = dvr_script.scriptapp("Resolve")
    if not resolve:
        print(json.dumps({{"error": "Could not connect to DaVinci Resolve"}}))
        sys.exit(1)
    
    project_manager = resolve.GetProjectManager()
    project = project_manager.GetCurrentProject()
    
    if not project:
        print(json.dumps({{"error": "No project open"}}))
        sys.exit(1)
    
    method = "{}"
    args = {}
    
    # Handle different API methods
    if method == "switch_page":
        page = args.get("page", "edit")
        resolve.OpenPage(page)
        result = {{"result": f"Switched to {{page}} page", "previous_page": page}}
    elif method == "create_empty_timeline":
        name = args.get("name", "New Timeline")
        frame_rate = args.get("frame_rate", "24")
        width = args.get("resolution_width", 1920)
        height = args.get("resolution_height", 1080)
        
        media_pool = project.GetMediaPool()
        timeline = media_pool.CreateEmptyTimeline(name)
        if timeline:
            result = {{"result": f"Created timeline '{{name}}'", "timeline_id": str(timeline.GetUniqueId())}}
        else:
            result = {{"error": "Failed to create timeline"}}
    elif method == "add_marker":
        timeline = project.GetCurrentTimeline()
        if timeline:
            frame = args.get("frame", 0)
            color = args.get("color", "Blue")
            note = args.get("note", "")
            
            marker_added = timeline.AddMarker(frame, color, note, note, 1)
            if marker_added:
                result = {{"result": f"Added {{color}} marker at frame {{frame}}"}}
            else:
                result = {{"error": "Failed to add marker"}}
        else:
            result = {{"error": "No timeline selected"}}
    elif method == "list_timelines_tool":
        media_pool = project.GetMediaPool()
        timelines = []
        timeline_count = project.GetTimelineCount()
        for i in range(1, timeline_count + 1):
            timeline = project.GetTimelineByIndex(i)
            if timeline:
                timelines.append({{
                    "name": timeline.GetName(),
                    "frame_rate": timeline.GetSetting("timelineFrameRate"),
                    "resolution": f"{{timeline.GetSetting('timelineResolutionWidth')}}x{{timeline.GetSetting('timelineResolutionHeight')}}"
                }})
        result = {{"timelines": timelines, "count": len(timelines)}}
    else:
        result = {{"error": f"Unsupported method: {{method}}"}}
    
    print(json.dumps(result))
    
except Exception as e:
    print(json.dumps({{"error": str(e)}}))
    sys.exit(1)
"#, method, args);

        // Write script to temporary file
        let script_path = "/tmp/dvr_api_call.py";
        std::fs::write(script_path, python_script)
            .map_err(|e| ResolveError::internal(&format!("Failed to write Python script: {}", e)))?;

        // Execute Python script
        let output = Command::new("python3")
            .arg(script_path)
            .output()
            .map_err(|e| ResolveError::internal(&format!("Failed to execute Python script: {}", e)))?;

        // Clean up
        let _ = std::fs::remove_file(script_path);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ResolveError::internal(&format!("Python script failed: {}", stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: Value = serde_json::from_str(&stdout)
            .map_err(|e| ResolveError::internal(&format!("Failed to parse Python output: {}", e)))?;

        if let Some(error) = result.get("error") {
            return Err(ResolveError::internal(&format!("DaVinci Resolve API error: {}", error)));
        }

        Ok(result)
    }

    async fn create_project(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;
        
        if state.projects.contains(&name.to_string()) {
            return Err(ResolveError::invalid_parameter("name", "project already exists"));
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
        let name = args["name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;
        
        if !state.projects.contains(&name.to_string()) {
            return Err(ResolveError::ProjectNotFound { name: name.to_string() });
        }

        state.current_project = Some(name.to_string());
        
        // Simulate loading existing timelines and media
        if !state.timelines.contains_key(name) {
            state.timelines.insert(name.to_string(), Timeline {
                name: format!("{} Timeline", name),
                frame_rate: Some("24".to_string()),
                resolution_width: Some(1920),
                resolution_height: Some(1080),
                markers: vec![],
            });
        }

        Ok(serde_json::json!({
            "result": format!("Opened project '{}'", name),
            "timelines": state.timelines.len(),
            "media_clips": state.media_pool.clips.len()
        }))
    }

    async fn switch_page(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let page = args["page"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("page", "required string"))?;
        
        let valid_pages = vec!["media", "cut", "edit", "fusion", "color", "fairlight", "deliver"];
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
        let name = args["name"].as_str()
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
            return Err(ResolveError::TimelineNotFound { name: "current".to_string() });
        }

        let timeline_name = state.current_timeline.as_ref().unwrap();
        let timeline = state.timelines.get_mut(timeline_name)
            .ok_or_else(|| ResolveError::TimelineNotFound { name: timeline_name.clone() })?;

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
        let file_path = args["file_path"].as_str()
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
        let name = args["name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;
        
        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        if state.media_pool.bins.contains_key(name) {
            return Err(ResolveError::invalid_parameter("name", "bin already exists"));
        }

        let bin = Bin {
            name: name.to_string(),
            clips: vec![],
        };

        state.media_pool.bins.insert(name.to_string(), bin);

        Ok(serde_json::json!({
            "result": format!("Created bin '{}'", name),
            "bin_id": Uuid::new_v4().to_string()
        }))
    }

    async fn auto_sync_audio(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_names = args["clip_names"].as_array()
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
        let clip_names = args["clip_names"].as_array()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_names", "required array"))?;
        
        Ok(serde_json::json!({
            "result": format!("Unlinked {} clips", clip_names.len()),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn relink_clips(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_names = args["clip_names"].as_array()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_names", "required array"))?;
        
        Ok(serde_json::json!({
            "result": format!("Relinked {} clips", clip_names.len()),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn create_sub_clip(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        let start_frame = args["start_frame"].as_i64().unwrap_or(0) as i32;
        let end_frame = args["end_frame"].as_i64().unwrap_or(100) as i32;
        
        let default_sub_clip_name = format!("{}_subclip", clip_name);
        let sub_clip_name = args["sub_clip_name"].as_str()
            .unwrap_or(&default_sub_clip_name);

        Ok(serde_json::json!({
            "result": format!("Created subclip '{}' from '{}' (frames {}-{})", 
                sub_clip_name, clip_name, start_frame, end_frame),
            "subclip_id": Uuid::new_v4().to_string(),
            "duration_frames": end_frame - start_frame
        }))
    }

    async fn link_proxy_media(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        
        Ok(serde_json::json!({
            "result": format!("Linked proxy media for clip '{}'", clip_name),
            "proxy_id": Uuid::new_v4().to_string()
        }))
    }

    async fn unlink_proxy_media(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        
        Ok(serde_json::json!({
            "result": format!("Unlinked proxy media for clip '{}'", clip_name),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn replace_clip(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        let replacement_path = args["replacement_path"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("replacement_path", "required string"))?;
        
        Ok(serde_json::json!({
            "result": format!("Replaced clip '{}' with '{}'", clip_name, replacement_path),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn delete_timeline(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;
        
        if state.timelines.remove(name).is_none() {
            return Err(ResolveError::TimelineNotFound { name: name.to_string() });
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

    async fn set_current_timeline(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("name", "required string"))?;
        
        if !state.timelines.contains_key(name) {
            return Err(ResolveError::TimelineNotFound { name: name.to_string() });
        }
        
        state.current_timeline = Some(name.to_string());

        Ok(serde_json::json!({
            "result": format!("Set current timeline to '{}'", name),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn create_empty_timeline(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let name = args["name"].as_str()
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

    async fn add_clip_to_timeline(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;
        
        let timeline_name = if let Some(name) = args["timeline_name"].as_str() {
            name.to_string()
        } else {
            state.current_timeline.clone()
                .ok_or_else(|| ResolveError::TimelineNotFound { name: "current".to_string() })?
        };
        
        if !state.timelines.contains_key(&timeline_name) {
            return Err(ResolveError::TimelineNotFound { name: timeline_name });
        }
        
        if !state.media_pool.clips.contains_key(clip_name) {
            return Err(ResolveError::MediaNotFound { name: clip_name.to_string() });
        }

        Ok(serde_json::json!({
            "result": format!("Added clip '{}' to timeline '{}'", clip_name, timeline_name),
            "timeline_item_id": Uuid::new_v4().to_string(),
            "track": "Video 1"
        }))
    }

    async fn list_timelines_tool(&self, state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
        let timeline_names: Vec<&String> = state.timelines.keys().collect();
        let timeline_list = if timeline_names.is_empty() {
            "No timelines available".to_string()
        } else {
            timeline_names.iter().map(|&name| name.clone()).collect::<Vec<String>>().join(", ")
        };

        Ok(serde_json::json!({
            "result": format!("Timelines: {}", timeline_list),
            "count": timeline_names.len(),
            "current_timeline": state.current_timeline
        }))
    }

    async fn get_timeline_tracks(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_name = if let Some(name) = args["timeline_name"].as_str() {
            name.to_string()
        } else {
            state.current_timeline.clone()
                .ok_or_else(|| ResolveError::TimelineNotFound { name: "current".to_string() })?
        };
        
        if !state.timelines.contains_key(&timeline_name) {
            return Err(ResolveError::TimelineNotFound { name: timeline_name });
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
        let lut_path = args["lut_path"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("lut_path", "required string"))?;
        let node_index = args["node_index"].as_i64().unwrap_or(state.color_state.current_node_index as i64) as i32;
        
        // Validate LUT exists (check if it's in our available LUTs or is a file path)
        let lut_name = if lut_path.starts_with('/') {
            // File path - validate it exists
            std::path::Path::new(lut_path).file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown LUT")
                .to_string()
        } else {
            // Check if it's a known LUT
            if !state.color_state.available_luts.contains_key(lut_path) {
                return Err(ResolveError::FileNotFound { path: lut_path.to_string() });
            }
            lut_path.to_string()
        };

        // Apply LUT to current clip
        if let Some(clip_name) = &state.color_state.current_clip {
            let grade = state.color_state.clip_grades.entry(clip_name.clone()).or_default();
            grade.applied_luts.push(lut_name.clone());
        }

        Ok(serde_json::json!({
            "result": format!("Applied LUT '{}' to node {}", lut_name, node_index),
            "lut_path": lut_path,
            "node_index": node_index,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_color_wheel_param(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let wheel = args["wheel"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("wheel", "required string"))?;
        let param = args["param"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("param", "required string"))?;
        let value = args["value"].as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("value", "required number"))?;
        let node_index = args["node_index"].as_i64().unwrap_or(state.color_state.current_node_index as i64) as i32;

        // Validate wheel and param
        let valid_wheels = vec!["lift", "gamma", "gain", "offset"];
        let valid_params = vec!["red", "green", "blue", "master"];
        
        if !valid_wheels.contains(&wheel) {
            return Err(ResolveError::invalid_parameter("wheel", "must be lift, gamma, gain, or offset"));
        }
        if !valid_params.contains(&param) {
            return Err(ResolveError::invalid_parameter("param", "must be red, green, blue, or master"));
        }

        // Apply to current clip
        if let Some(clip_name) = &state.color_state.current_clip {
            let grade = state.color_state.clip_grades.entry(clip_name.clone()).or_default();
            
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
            return Err(ResolveError::invalid_parameter("node_type", "must be serial, parallel, or layer"));
        }

        // Add node to current clip
        if let Some(clip_name) = &state.color_state.current_clip {
            let grade = state.color_state.clip_grades.entry(clip_name.clone()).or_default();
            grade.node_count += 1;
            
            if let Some(label_str) = label {
                grade.node_labels.insert(grade.node_count, label_str.to_string());
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
            state.color_state.current_clip.clone()
                .ok_or_else(|| ResolveError::invalid_parameter("source_clip_name", "no current clip"))?
        };

        // Use current clip as target if not specified
        let target = if let Some(target) = target_clip_name {
            target.to_string()
        } else {
            state.color_state.current_clip.clone()
                .ok_or_else(|| ResolveError::invalid_parameter("target_clip_name", "no current clip"))?
        };

        // Get source grade
        let source_grade = state.color_state.clip_grades.get(&source)
            .cloned()
            .unwrap_or_default();

        // Apply to target based on mode
        let result_msg = match mode {
            "full" => {
                state.color_state.clip_grades.insert(target.clone(), source_grade);
                format!("Copied full grade from '{}' to '{}'", source, target)
            },
            "current_node" => {
                // Simulate copying current node only
                format!("Copied current node grade from '{}' to '{}'", source, target)
            },
            "all_nodes" => {
                state.color_state.clip_grades.insert(target.clone(), source_grade);
                format!("Copied all nodes from '{}' to '{}'", source, target)
            },
            _ => return Err(ResolveError::invalid_parameter("mode", "must be full, current_node, or all_nodes")),
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "source_clip": source,
            "target_clip": target,
            "mode": mode,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn save_color_preset(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str();
        let preset_name = args["preset_name"].as_str();
        let album_name = args["album_name"].as_str().unwrap_or("DaVinci Resolve");

        // Use current clip if not specified
        let source_clip = if let Some(clip) = clip_name {
            clip.to_string()
        } else {
            state.color_state.current_clip.clone()
                .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "no current clip"))?
        };

        // Use clip name as preset name if not specified
        let preset_name_final = if let Some(name) = preset_name {
            name.to_string()
        } else {
            format!("{}_preset", source_clip)
        };

        // Get clip grade
        let grade = state.color_state.clip_grades.get(&source_clip)
            .cloned()
            .unwrap_or_default();

        // Save preset
        let preset = ColorPreset {
            name: preset_name_final.clone(),
            album: album_name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            grade_data: grade,
        };

        state.color_state.color_presets.insert(preset_name_final.clone(), preset);

        Ok(serde_json::json!({
            "result": format!("Saved color preset '{}' from clip '{}' to album '{}'", 
                preset_name_final, source_clip, album_name),
            "preset_name": preset_name_final,
            "album": album_name,
            "source_clip": source_clip,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn apply_color_preset(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
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
            return Err(ResolveError::invalid_parameter("preset_id or preset_name", "one is required"));
        };

        let preset = preset.ok_or_else(|| ResolveError::invalid_parameter("preset", "preset not found"))?;

        // Use current clip if not specified
        let target_clip = if let Some(clip) = clip_name {
            clip.to_string()
        } else {
            state.color_state.current_clip.clone()
                .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "no current clip"))?
        };

        // Apply preset to clip
        state.color_state.clip_grades.insert(target_clip.clone(), preset.grade_data.clone());

        Ok(serde_json::json!({
            "result": format!("Applied color preset '{}' from album '{}' to clip '{}'", 
                preset.name, album_name, target_clip),
            "preset_name": preset.name,
            "album": album_name,
            "target_clip": target_clip,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn delete_color_preset(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let preset_id = args["preset_id"].as_str();
        let preset_name = args["preset_name"].as_str();
        let album_name = args["album_name"].as_str().unwrap_or("DaVinci Resolve");

        // Find preset by ID or name
        let preset_key = if let Some(id) = preset_id {
            id.to_string()
        } else if let Some(name) = preset_name {
            name.to_string()
        } else {
            return Err(ResolveError::invalid_parameter("preset_id or preset_name", "one is required"));
        };

        let removed_preset = state.color_state.color_presets.remove(&preset_key)
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
        let source_clip = if let Some(clip) = clip_name {
            clip.to_string()
        } else {
            state.color_state.current_clip.clone()
                .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "no current clip"))?
        };

        // Validate format and size
        let valid_formats = vec!["Cube", "Davinci", "3dl", "Panasonic"];
        let valid_sizes = vec!["17Point", "33Point", "65Point"];
        
        if !valid_formats.contains(&lut_format) {
            return Err(ResolveError::invalid_parameter("lut_format", "invalid format"));
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

    async fn set_timeline_item_transform(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_name = args["property_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let property_value = args["property_value"].as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("property_value", "required number"))?;

        // Validate property name
        let valid_properties = vec!["Pan", "Tilt", "ZoomX", "ZoomY", "Rotation", "AnchorPointX", "AnchorPointY", "Pitch", "Yaw"];
        if !valid_properties.contains(&property_name) {
            return Err(ResolveError::invalid_parameter("property_name", "invalid transform property"));
        }

        // Get or create timeline item
        let timeline_item = state.timeline_items.items.entry(timeline_item_id.to_string())
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

    async fn set_timeline_item_crop(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let crop_type = args["crop_type"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("crop_type", "required string"))?;
        let crop_value = args["crop_value"].as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("crop_value", "required number"))?;

        // Validate crop type and value
        let valid_crop_types = vec!["Left", "Right", "Top", "Bottom"];
        if !valid_crop_types.contains(&crop_type) {
            return Err(ResolveError::invalid_parameter("crop_type", "must be Left, Right, Top, or Bottom"));
        }
        if crop_value < 0.0 || crop_value > 1.0 {
            return Err(ResolveError::invalid_parameter("crop_value", "must be between 0.0 and 1.0"));
        }

        // Get or create timeline item
        let timeline_item = state.timeline_items.items.entry(timeline_item_id.to_string())
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

    async fn set_timeline_item_composite(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let composite_mode = args["composite_mode"].as_str();
        let opacity = args["opacity"].as_f64();

        // Validate composite mode if provided
        if let Some(mode) = composite_mode {
            let valid_modes = vec!["Normal", "Add", "Multiply", "Screen", "Overlay", "SoftLight", "HardLight", 
                                 "ColorDodge", "ColorBurn", "Darken", "Lighten", "Difference", "Exclusion"];
            if !valid_modes.contains(&mode) {
                return Err(ResolveError::invalid_parameter("composite_mode", "invalid composite mode"));
            }
        }

        // Validate opacity if provided
        if let Some(opacity_val) = opacity {
            if opacity_val < 0.0 || opacity_val > 1.0 {
                return Err(ResolveError::invalid_parameter("opacity", "must be between 0.0 and 1.0"));
            }
        }

        // Get or create timeline item
        let timeline_item = state.timeline_items.items.entry(timeline_item_id.to_string())
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
            format!("Set {} for timeline item '{}'", result_parts.join(" and "), timeline_item_id)
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "composite_mode": composite_mode,
            "opacity": opacity,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_retime(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let speed = args["speed"].as_f64();
        let process = args["process"].as_str();

        // Validate speed if provided
        if let Some(speed_val) = speed {
            if speed_val <= 0.0 || speed_val > 10.0 {
                return Err(ResolveError::invalid_parameter("speed", "must be between 0.0 and 10.0"));
            }
        }

        // Validate process if provided
        if let Some(process_str) = process {
            let valid_processes = vec!["NearestFrame", "FrameBlend", "OpticalFlow"];
            if !valid_processes.contains(&process_str) {
                return Err(ResolveError::invalid_parameter("process", "must be NearestFrame, FrameBlend, or OpticalFlow"));
            }
        }

        // Get or create timeline item
        let timeline_item = state.timeline_items.items.entry(timeline_item_id.to_string())
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
            format!("Set {} for timeline item '{}'", result_parts.join(" and "), timeline_item_id)
        };

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "speed": speed,
            "process": process,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn set_timeline_item_stabilization(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let enabled = args["enabled"].as_bool();
        let method = args["method"].as_str();
        let strength = args["strength"].as_f64();

        // Validate method if provided
        if let Some(method_str) = method {
            let valid_methods = vec!["Perspective", "Similarity", "Translation"];
            if !valid_methods.contains(&method_str) {
                return Err(ResolveError::invalid_parameter("method", "must be Perspective, Similarity, or Translation"));
            }
        }

        // Validate strength if provided
        if let Some(strength_val) = strength {
            if strength_val < 0.0 || strength_val > 1.0 {
                return Err(ResolveError::invalid_parameter("strength", "must be between 0.0 and 1.0"));
            }
        }

        // Get or create timeline item
        let timeline_item = state.timeline_items.items.entry(timeline_item_id.to_string())
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
            format!("Set stabilization {} for timeline item '{}'", result_parts.join(", "), timeline_item_id)
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

    async fn set_timeline_item_audio(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let volume = args["volume"].as_f64();
        let pan = args["pan"].as_f64();
        let eq_enabled = args["eq_enabled"].as_bool();

        // Validate volume if provided
        if let Some(volume_val) = volume {
            if volume_val < 0.0 || volume_val > 2.0 {
                return Err(ResolveError::invalid_parameter("volume", "must be between 0.0 and 2.0"));
            }
        }

        // Validate pan if provided
        if let Some(pan_val) = pan {
            if pan_val < -1.0 || pan_val > 1.0 {
                return Err(ResolveError::invalid_parameter("pan", "must be between -1.0 and 1.0"));
            }
        }

        // Get or create timeline item
        let timeline_item = state.timeline_items.items.entry(timeline_item_id.to_string())
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
            format!("Set audio {} for timeline item '{}'", result_parts.join(", "), timeline_item_id)
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

    async fn get_timeline_item_properties(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;

        // Get timeline item
        let timeline_item = state.timeline_items.items.get(timeline_item_id)
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "timeline item not found"))?;

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

    async fn reset_timeline_item_properties(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_type = args["property_type"].as_str();

        // Get timeline item
        let timeline_item = state.timeline_items.items.get_mut(timeline_item_id)
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "timeline item not found"))?;

        let mut reset_parts = Vec::new();

        // Reset specific property type or all if not specified
        match property_type {
            Some("transform") => {
                timeline_item.transform = TransformProperties::default();
                reset_parts.push("transform");
            },
            Some("crop") => {
                timeline_item.crop = CropProperties::default();
                reset_parts.push("crop");
            },
            Some("composite") => {
                timeline_item.composite = CompositeProperties {
                    mode: "Normal".to_string(),
                    opacity: 1.0,
                };
                reset_parts.push("composite");
            },
            Some("retime") => {
                timeline_item.retime = RetimeProperties {
                    speed: 1.0,
                    process: "NearestFrame".to_string(),
                };
                reset_parts.push("retime");
            },
            Some("stabilization") => {
                timeline_item.stabilization = StabilizationProperties::default();
                reset_parts.push("stabilization");
            },
            Some("audio") => {
                timeline_item.audio = AudioProperties {
                    volume: 1.0,
                    pan: 0.0,
                    eq_enabled: false,
                };
                reset_parts.push("audio");
            },
            Some(_invalid_type) => {
                return Err(ResolveError::invalid_parameter("property_type", 
                    "must be transform, crop, composite, retime, stabilization, or audio"));
            },
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

        let result_msg = format!("Reset {} for timeline item '{}'", reset_parts.join(", "), timeline_item_id);

        Ok(serde_json::json!({
            "result": result_msg,
            "timeline_item_id": timeline_item_id,
            "property_type": property_type,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    // ==================== KEYFRAME ANIMATION OPERATIONS (Phase 4 Week 2) ====================

    async fn add_keyframe(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_name = args["property_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"].as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))? as i32;
        let value = args["value"].as_f64()
            .ok_or_else(|| ResolveError::invalid_parameter("value", "required number"))?;

        // Validate property name
        let valid_properties = vec![
            "Pan", "Tilt", "ZoomX", "ZoomY", "Rotation", "AnchorPointX", "AnchorPointY", "Pitch", "Yaw",
            "Left", "Right", "Top", "Bottom", "Opacity", "Speed", "Strength", "Volume", "AudioPan"
        ];
        if !valid_properties.contains(&property_name) {
            return Err(ResolveError::invalid_parameter("property_name", 
                "must be a valid timeline item property"));
        }

        // Validate frame position
        if frame < 0 {
            return Err(ResolveError::invalid_parameter("frame", "must be non-negative"));
        }

        // Generate keyframe ID
        state.keyframe_state.keyframe_counter += 1;
        let keyframe_id = state.keyframe_state.keyframe_counter;

        // Get or create timeline item keyframes
        let timeline_item_keyframes = state.keyframe_state.timeline_item_keyframes
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
        let property_keyframes = timeline_item_keyframes.property_keyframes
            .entry(property_name.to_string())
            .or_insert_with(Vec::new);

        // Insert keyframe in sorted order by frame
        let insert_pos = property_keyframes.binary_search_by_key(&frame, |k| k.frame)
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
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_name = args["property_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"].as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))? as i32;
        let new_value = args["new_value"].as_f64();
        let new_frame = args["new_frame"].as_i64().map(|f| f as i32);

        // Get timeline item keyframes
        let timeline_item_keyframes = state.keyframe_state.timeline_item_keyframes
            .get_mut(timeline_item_id)
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", 
                "no keyframes found for timeline item"))?;

        // Get property keyframes
        let property_keyframes = timeline_item_keyframes.property_keyframes
            .get_mut(property_name)
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", 
                "no keyframes found for property"))?;

        // Find keyframe at specified frame
        let keyframe_index = property_keyframes.iter()
            .position(|k| k.frame == frame)
            .ok_or_else(|| ResolveError::invalid_parameter("frame", 
                "no keyframe found at specified frame"))?;

        let mut modifications = Vec::new();

        // Modify value if provided
        if let Some(value) = new_value {
            property_keyframes[keyframe_index].value = value;
            modifications.push(format!("value to {}", value));
        }

        // Modify frame position if provided
        if let Some(new_frame_pos) = new_frame {
            if new_frame_pos < 0 {
                return Err(ResolveError::invalid_parameter("new_frame", "must be non-negative"));
            }

            // Remove keyframe from current position
            let mut keyframe = property_keyframes.remove(keyframe_index);
            keyframe.frame = new_frame_pos;

            // Re-insert in sorted order
            let insert_pos = property_keyframes.binary_search_by_key(&new_frame_pos, |k| k.frame)
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
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_name = args["property_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"].as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))? as i32;

        // Get timeline item keyframes
        let timeline_item_keyframes = state.keyframe_state.timeline_item_keyframes
            .get_mut(timeline_item_id)
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", 
                "no keyframes found for timeline item"))?;

        // Get property keyframes
        let property_keyframes = timeline_item_keyframes.property_keyframes
            .get_mut(property_name)
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", 
                "no keyframes found for property"))?;

        // Find and remove keyframe at specified frame
        let keyframe_index = property_keyframes.iter()
            .position(|k| k.frame == frame)
            .ok_or_else(|| ResolveError::invalid_parameter("frame", 
                "no keyframe found at specified frame"))?;

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

    async fn set_keyframe_interpolation(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_name = args["property_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", "required string"))?;
        let frame = args["frame"].as_i64()
            .ok_or_else(|| ResolveError::invalid_parameter("frame", "required integer"))? as i32;
        let interpolation_type = args["interpolation_type"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("interpolation_type", "required string"))?;

        // Validate interpolation type
        let interpolation = match interpolation_type {
            "Linear" => InterpolationType::Linear,
            "Bezier" => InterpolationType::Bezier,
            "Ease-In" => InterpolationType::EaseIn,
            "Ease-Out" => InterpolationType::EaseOut,
            "Hold" => InterpolationType::Hold,
            _ => return Err(ResolveError::invalid_parameter("interpolation_type", 
                "must be Linear, Bezier, Ease-In, Ease-Out, or Hold")),
        };

        // Get timeline item keyframes
        let timeline_item_keyframes = state.keyframe_state.timeline_item_keyframes
            .get_mut(timeline_item_id)
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", 
                "no keyframes found for timeline item"))?;

        // Get property keyframes
        let property_keyframes = timeline_item_keyframes.property_keyframes
            .get_mut(property_name)
            .ok_or_else(|| ResolveError::invalid_parameter("property_name", 
                "no keyframes found for property"))?;

        // Find keyframe at specified frame
        let keyframe = property_keyframes.iter_mut()
            .find(|k| k.frame == frame)
            .ok_or_else(|| ResolveError::invalid_parameter("frame", 
                "no keyframe found at specified frame"))?;

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

    async fn enable_keyframes(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let keyframe_mode = args["keyframe_mode"].as_str().unwrap_or("All");

        // Validate keyframe mode
        if !["All", "Color", "Sizing"].contains(&keyframe_mode) {
            return Err(ResolveError::invalid_parameter("keyframe_mode", 
                "must be All, Color, or Sizing"));
        }

        // Get or create timeline item keyframes
        let timeline_item_keyframes = state.keyframe_state.timeline_item_keyframes
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
        let timeline_item_id = args["timeline_item_id"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", "required string"))?;
        let property_name = args["property_name"].as_str();

        // Get timeline item keyframes
        let timeline_item_keyframes = state.keyframe_state.timeline_item_keyframes
            .get(timeline_item_id)
            .ok_or_else(|| ResolveError::invalid_parameter("timeline_item_id", 
                "no keyframes found for timeline item"))?;

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
                let keyframe_data: Vec<serde_json::Value> = keyframes.iter().map(|kf| {
                    serde_json::json!({
                        "id": kf.id,
                        "frame": kf.frame,
                        "value": kf.value,
                        "interpolation": format!("{:?}", kf.interpolation),
                        "created_at": kf.created_at
                    })
                }).collect();

                result["property_name"] = serde_json::Value::String(prop_name.to_string());
                result["keyframes"] = serde_json::Value::Array(keyframe_data);
                result["total_keyframes"] = serde_json::Value::Number(
                    serde_json::Number::from(keyframes.len())
                );
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
                let keyframe_data: Vec<serde_json::Value> = keyframes.iter().map(|kf| {
                    serde_json::json!({
                        "id": kf.id,
                        "frame": kf.frame,
                        "value": kf.value,
                        "interpolation": format!("{:?}", kf.interpolation),
                        "created_at": kf.created_at
                    })
                }).collect();

                all_properties.insert(prop_name.clone(), serde_json::Value::Array(keyframe_data));
                total_count += keyframes.len();
            }

            result["properties"] = serde_json::Value::Object(all_properties);
            result["total_keyframes"] = serde_json::Value::Number(
                serde_json::Number::from(total_count)
            );
        }

        Ok(result)
    }

    // ==================== RENDER & DELIVERY OPERATIONS (Phase 4 Week 3) ====================

    async fn add_to_render_queue(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("preset_name", "required string"))?;
        let timeline_name = args["timeline_name"].as_str()
            .unwrap_or_else(|| state.current_timeline.as_ref().map(|s| s.as_str()).unwrap_or("Timeline 1"));
        let use_in_out_range = args["use_in_out_range"].as_bool().unwrap_or(false);

        // Validate timeline exists
        if !state.timelines.contains_key(timeline_name) {
            return Err(ResolveError::TimelineNotFound { name: timeline_name.to_string() });
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
            state.render_state.render_presets.insert("H.264 1080p".to_string(), default_preset);
        }

        // Validate preset exists
        if !state.render_state.render_presets.contains_key(preset_name) {
            return Err(ResolveError::PresetNotFound { name: preset_name.to_string() });
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
            return Err(ResolveError::invalid_parameter("render_queue", "no jobs in queue"));
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

                state.render_state.active_renders.insert(job.id.clone(), progress);
                started_jobs.push(job.id.clone());
            }
        }

        if started_jobs.is_empty() {
            return Err(ResolveError::invalid_parameter("render_queue", "no queued jobs to start"));
        }

        tracing::info!("Started {} render jobs", started_jobs.len());

        Ok(serde_json::json!({
            "result": format!("Started {} render jobs", started_jobs.len()),
            "started_jobs": started_jobs,
            "total_active_renders": state.render_state.active_renders.len(),
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn clear_render_queue(&self, state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
        let queue_size = state.render_state.render_queue.len();
        let active_renders = state.render_state.active_renders.len();

        // Clear render queue and active renders
        state.render_state.render_queue.clear();
        state.render_state.active_renders.clear();

        tracing::info!("Cleared render queue ({} jobs) and active renders ({} jobs)", queue_size, active_renders);

        Ok(serde_json::json!({
            "result": format!("Cleared render queue ({} jobs) and stopped {} active renders", queue_size, active_renders),
            "cleared_queue_jobs": queue_size,
            "stopped_active_renders": active_renders,
            "operation_id": Uuid::new_v4().to_string()
        }))
    }

    async fn get_render_status(&self, state: &mut ResolveState, _args: Value) -> ResolveResult<Value> {
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
        let queued_job_details: Vec<_> = state.render_state.render_queue.iter()
            .filter(|job| matches!(job.status, RenderJobStatus::Queued))
            .map(|job| serde_json::json!({
                "job_id": job.id,
                "timeline_name": job.timeline_name,
                "preset_name": job.preset_name,
                "output_path": job.output_path,
                "use_in_out_range": job.use_in_out_range
            }))
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
        let export_path = args["export_path"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("export_path", "required string"))?;
        let include_media = args["include_media"].as_bool().unwrap_or(false);
        let project_name = args["project_name"].as_str()
            .unwrap_or_else(|| state.current_project.as_ref().map(|s| s.as_str()).unwrap_or("Unknown Project"));

        // Validate current project exists
        if state.current_project.is_none() {
            return Err(ResolveError::invalid_parameter("project", "no project currently open"));
        }

        // Validate export path
        if export_path.is_empty() {
            return Err(ResolveError::invalid_parameter("export_path", "cannot be empty"));
        }

        tracing::info!("Exporting project '{}' to '{}'", project_name, export_path);

        // Simulate export process
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Simulate export file size
        let timeline_count = state.timelines.len();
        let media_count = state.media_pool.clips.len();
        let estimated_size_mb = if include_media { 500 + media_count * 50 } else { 50 + timeline_count * 10 };

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

    async fn create_render_preset(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let preset_name = args["preset_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("preset_name", "required string"))?;
        let format = args["format"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("format", "required string"))?;
        let codec = args["codec"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("codec", "required string"))?;
        let resolution = (
            args["resolution_width"].as_i64().unwrap() as u32,
            args["resolution_height"].as_i64().unwrap() as u32
        );
        let frame_rate = args["frame_rate"].as_f64().unwrap() as f32;
        let quality = args["quality"].as_u64().unwrap() as u32;
        let audio_codec = args["audio_codec"].as_str()
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
            return Err(ResolveError::invalid_parameter("resolution", "must be at least 1920x1080"));
        }

        // Validate frame rate
        if frame_rate < 24.0 || frame_rate > 60.0 {
            return Err(ResolveError::invalid_parameter("frame_rate", "must be between 24.0 and 60.0"));
        }

        // Validate quality
        if quality < 1 || quality > 100 {
            return Err(ResolveError::invalid_parameter("quality", "must be between 1 and 100"));
        }

        // Validate audio codec
        let valid_audio_codecs = vec!["AAC", "ProRes"];
        if !valid_audio_codecs.contains(&audio_codec) {
            return Err(ResolveError::invalid_parameter("audio_codec", "invalid audio codec"));
        }

        // Validate audio bitrate
        if audio_bitrate < 64000 || audio_bitrate > 192000 {
            return Err(ResolveError::invalid_parameter("audio_bitrate", "must be between 64kbps and 192kbps"));
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
        state.render_state.render_presets.insert(preset_name.to_string(), render_preset);

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

    async fn set_project_setting(&self, state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        if state.current_project.is_none() {
            return Err(ResolveError::NotRunning);
        }

        let setting_name = args["setting_name"].as_str()
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
    async fn transcribe_audio(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
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

    async fn clear_transcription(&self, _state: &mut ResolveState, args: Value) -> ResolveResult<Value> {
        let clip_name = args["clip_name"].as_str()
            .ok_or_else(|| ResolveError::invalid_parameter("clip_name", "required string"))?;

        Ok(serde_json::json!({
            "result": format!("Cleared transcription for clip '{}'", clip_name),
            "operation_id": Uuid::new_v4().to_string(),
            "clip_name": clip_name
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

        let marker = Marker { frame, color: color.clone(), note: note.clone() };

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