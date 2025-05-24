use std::sync::Arc;
use pyo3::prelude::*;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::{debug, error, instrument};

use crate::error::{ResolveError, ResolveResult};

/// Python bridge for communicating with DaVinci Resolve Python API
#[derive(Debug)]
pub struct ResolveBridge {
    /// Python module containing the bridge implementation
    py_bridge: Arc<Mutex<Option<PyObject>>>,
    /// Whether the bridge is initialized
    initialized: Arc<Mutex<bool>>,
}

impl ResolveBridge {
    /// Create a new ResolveBridge instance
    pub fn new() -> Self {
        Self {
            py_bridge: Arc::new(Mutex::new(None)),
            initialized: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Initialize the Python bridge
    #[instrument(skip(self))]
    pub async fn initialize(&self) -> ResolveResult<()> {
        let mut initialized = self.initialized.lock().await;
        if *initialized {
            return Ok(());
        }
        
        let mut bridge = self.py_bridge.lock().await;
        
        Python::with_gil(|py| -> ResolveResult<()> {
            // Import the Python bridge module
            let bridge_code = include_str!("resolve_bridge.py");
            let bridge_module = PyModule::from_code(py, bridge_code, "resolve_bridge.py", "resolve_bridge")?;
            
            // Create bridge instance
            let bridge_class = bridge_module.getattr("ResolveBridge")?;
            let bridge_instance = bridge_class.call0()?;
            
            *bridge = Some(bridge_instance.into());
            *initialized = true;
            
            debug!("Python bridge initialized successfully");
            Ok(())
        })
    }
    
    /// Call a DaVinci Resolve API method through the Python bridge
    #[instrument(skip(self, args))]
    pub async fn call_api(&self, method: &str, args: Value) -> ResolveResult<Value> {
        // Ensure bridge is initialized
        self.initialize().await?;
        
        let bridge = self.py_bridge.lock().await;
        let bridge_ref = bridge.as_ref()
            .ok_or_else(|| ResolveError::internal("Bridge not initialized"))?;
        
        Python::with_gil(|py| -> ResolveResult<Value> {
            debug!("Calling API method: {} with args: {}", method, args);
            
            // Convert Rust Value to Python object
            let py_args = pythonize::pythonize(py, &args)
                .map_err(|e| ResolveError::internal(format!("Failed to convert args to Python: {}", e)))?;
            
            // Call the bridge method
            let result = bridge_ref.call_method1(py, "call_api", (method, py_args))?;
            
            // Convert Python result back to Rust Value
            let rust_result: Value = pythonize::depythonize(result.as_ref(py))
                .map_err(|e| ResolveError::internal(format!("Failed to convert result from Python: {}", e)))?;
            
            debug!("API call successful: {} -> {}", method, rust_result);
            Ok(rust_result)
        })
    }
    
    /// Check if DaVinci Resolve is running
    #[instrument(skip(self))]
    pub async fn is_resolve_running(&self) -> ResolveResult<bool> {
        match self.call_api("is_running", Value::Null).await {
            Ok(Value::Bool(running)) => Ok(running),
            Ok(_) => Ok(false),
            Err(_) => Ok(false),
        }
    }
    
    /// Get the current project name
    #[instrument(skip(self))]
    pub async fn get_current_project_name(&self) -> ResolveResult<Option<String>> {
        let result = self.call_api("get_current_project_name", Value::Null).await?;
        match result {
            Value::String(name) => Ok(Some(name)),
            Value::Null => Ok(None),
            _ => Err(ResolveError::internal("Unexpected response format")),
        }
    }
    
    /// Get list of all timelines in current project
    #[instrument(skip(self))]
    pub async fn get_timelines(&self) -> ResolveResult<Vec<String>> {
        let result = self.call_api("get_timelines", Value::Null).await?;
        match result {
            Value::Array(timelines) => {
                let names: Result<Vec<String>, _> = timelines
                    .into_iter()
                    .map(|v| match v {
                        Value::String(name) => Ok(name),
                        _ => Err(ResolveError::internal("Invalid timeline name format")),
                    })
                    .collect();
                names
            }
            _ => Err(ResolveError::internal("Unexpected response format")),
        }
    }
}

impl Default for ResolveBridge {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export for convenience
pub use self::ResolveBridge as Bridge; 