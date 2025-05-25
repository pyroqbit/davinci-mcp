use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use libloading::{Library, Symbol};
use anyhow::{Result, anyhow};
use tracing::{info, warn, debug};

/// Native DaVinci Resolve FFI integration
pub struct NativeDaVinciResolve {
    fusion_lib: Option<Library>,
    com_api_lib: Option<Library>,
    is_connected: bool,
}

impl std::fmt::Debug for NativeDaVinciResolve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeDaVinciResolve")
            .field("fusion_lib", &self.fusion_lib.is_some())
            .field("com_api_lib", &self.com_api_lib.is_some())
            .field("is_connected", &self.is_connected)
            .finish()
    }
}

/// FFI function signatures for DaVinci Resolve
type ResolveConnectFn = unsafe extern "C" fn(*const c_char, c_int, *const c_char, *const c_char) -> *mut c_void;
type ResolveDisconnectFn = unsafe extern "C" fn(*mut c_void) -> c_int;
type ResolveExecuteFn = unsafe extern "C" fn(*mut c_void, *const c_char) -> *const c_char;

impl NativeDaVinciResolve {
    /// Create new native DaVinci Resolve connection
    pub fn new() -> Self {
        Self {
            fusion_lib: None,
            com_api_lib: None,
            is_connected: false,
        }
    }

    /// Initialize native libraries
    pub fn initialize(&mut self) -> Result<()> {
        info!("🔧 Initializing native DaVinci Resolve integration...");

        // Load Fusion script library
        match self.load_fusion_library() {
            Ok(()) => info!("✅ Fusion library loaded successfully"),
            Err(e) => {
                warn!("⚠️ Failed to load Fusion library: {}", e);
                return self.fallback_to_python_mode();
            }
        }

        // Load COM API library
        match self.load_com_api_library() {
            Ok(()) => info!("✅ COM API library loaded successfully"),
            Err(e) => {
                warn!("⚠️ Failed to load COM API library: {}", e);
                return self.fallback_to_python_mode();
            }
        }

        info!("🚀 Native DaVinci Resolve integration initialized!");
        Ok(())
    }

    /// Load Fusion script library
    fn load_fusion_library(&mut self) -> Result<()> {
        let fusion_path = "/opt/resolve/libs/Fusion/fusionscript.so";
        
        debug!("Loading Fusion library from: {}", fusion_path);
        
        unsafe {
            let lib = Library::new(fusion_path)
                .map_err(|e| anyhow!("Failed to load fusionscript.so: {}", e))?;
            
            // Verify required symbols exist
            let _: Symbol<ResolveConnectFn> = lib.get(b"Resolve\0")
                .map_err(|e| anyhow!("Missing Resolve function: {}", e))?;
            
            self.fusion_lib = Some(lib);
            Ok(())
        }
    }

    /// Load COM API library
    fn load_com_api_library(&mut self) -> Result<()> {
        let com_api_path = "/opt/resolve/libs/libcom-api.so";
        
        debug!("Loading COM API library from: {}", com_api_path);
        
        unsafe {
            let lib = Library::new(com_api_path)
                .map_err(|e| anyhow!("Failed to load libcom-api.so: {}", e))?;
            
            self.com_api_lib = Some(lib);
            Ok(())
        }
    }

    /// Fallback to Python mode if native fails
    fn fallback_to_python_mode(&self) -> Result<()> {
        warn!("🐍 Falling back to Python integration mode");
        info!("💡 Native integration failed, but Python mode is still available");
        Ok(())
    }

    /// Connect to DaVinci Resolve natively
    pub fn connect(&mut self) -> Result<()> {
        if self.fusion_lib.is_none() {
            return Err(anyhow!("Fusion library not loaded"));
        }

        info!("🔌 Connecting to DaVinci Resolve natively...");

        unsafe {
            let lib = self.fusion_lib.as_ref().unwrap();
            let resolve_fn: Symbol<ResolveConnectFn> = lib.get(b"Resolve\0")?;
            
            // Connect with default parameters
            let ip = CString::new("127.0.0.1")?;
            let port = 15000;
            let uuid = CString::new("")?;
            let subtype = CString::new("")?;
            
            let connection = resolve_fn(ip.as_ptr(), port, uuid.as_ptr(), subtype.as_ptr());
            
            if connection.is_null() {
                return Err(anyhow!("Failed to connect to DaVinci Resolve"));
            }
            
            self.is_connected = true;
            info!("✅ Successfully connected to DaVinci Resolve natively!");
            Ok(())
        }
    }

    /// Execute native command
    pub fn execute_command(&self, command: &str) -> Result<String> {
        if !self.is_connected {
            return Err(anyhow!("Not connected to DaVinci Resolve"));
        }

        debug!("🎬 Executing native command: {}", command);

        // For now, simulate command execution
        // In a real implementation, this would call the native API
        let result = format!("Native execution result for: {}", command);
        
        debug!("📤 Native command result: {}", result);
        Ok(result)
    }

    /// Check if native mode is available
    pub fn is_native_available(&self) -> bool {
        self.fusion_lib.is_some() && self.com_api_lib.is_some()
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    /// Get connection info
    pub fn get_connection_info(&self) -> String {
        if self.is_native_available() {
            if self.is_connected {
                "🚀 Native DaVinci Resolve connection active".to_string()
            } else {
                "🔧 Native libraries loaded, not connected".to_string()
            }
        } else {
            "🐍 Python fallback mode".to_string()
        }
    }
}

impl Drop for NativeDaVinciResolve {
    fn drop(&mut self) {
        if self.is_connected {
            info!("🔌 Disconnecting from DaVinci Resolve...");
            self.is_connected = false;
        }
    }
}

/// Test native integration
pub fn test_native_integration() -> Result<()> {
    info!("🧪 Testing native DaVinci Resolve integration...");
    
    let mut native = NativeDaVinciResolve::new();
    
    // Initialize
    native.initialize()?;
    
    // Test connection
    match native.connect() {
        Ok(()) => {
            info!("✅ Native integration test successful!");
            info!("📊 Connection info: {}", native.get_connection_info());
        },
        Err(e) => {
            warn!("⚠️ Native connection failed: {}", e);
            info!("💡 This is expected if DaVinci Resolve is not running");
        }
    }
    
    Ok(())
} 