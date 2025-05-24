use thiserror::Error;

/// Comprehensive error types for DaVinci Resolve MCP operations
#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("DaVinci Resolve is not running")]
    NotRunning,
    
    #[error("Project not found: {name}")]
    ProjectNotFound { name: String },
    
    #[error("Timeline not found: {name}")]
    TimelineNotFound { name: String },
    
    #[error("Media clip not found: {name}")]
    MediaNotFound { name: String },
    
    #[error("Bin not found: {name}")]
    BinNotFound { name: String },
    
    #[error("Invalid timeline item ID: {id}")]
    InvalidTimelineItemId { id: String },
    
    #[error("Invalid node index: {index}")]
    InvalidNodeIndex { index: i32 },
    
    #[error("Python bridge error: {0}")]
    PythonBridge(#[from] pyo3::PyErr),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("API call failed: {method} - {message}")]
    ApiCall { method: String, message: String },
    
    #[error("Invalid parameter: {param} - {reason}")]
    InvalidParameter { param: String, reason: String },
    
    #[error("Operation not supported: {operation}")]
    NotSupported { operation: String },
    
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },
    
    #[error("Timeout during operation: {operation}")]
    Timeout { operation: String },
    
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl ResolveError {
    /// Create a new API call error
    pub fn api_call(method: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ApiCall {
            method: method.into(),
            message: message.into(),
        }
    }
    
    /// Create a new invalid parameter error
    pub fn invalid_parameter(param: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidParameter {
            param: param.into(),
            reason: reason.into(),
        }
    }
    
    /// Create a new not supported error
    pub fn not_supported(operation: impl Into<String>) -> Self {
        Self::NotSupported {
            operation: operation.into(),
        }
    }
    
    /// Create a new internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}

/// Result type alias for DaVinci Resolve operations
pub type ResolveResult<T> = Result<T, ResolveError>;

/// Convert ResolveError to MCP JSON-RPC error
impl From<ResolveError> for mcp::JsonRpcError {
    fn from(err: ResolveError) -> Self {
        let (code, message) = match err {
            ResolveError::NotRunning => (-32001, err.to_string()),
            ResolveError::ProjectNotFound { .. } => (-32002, err.to_string()),
            ResolveError::TimelineNotFound { .. } => (-32003, err.to_string()),
            ResolveError::MediaNotFound { .. } => (-32004, err.to_string()),
            ResolveError::BinNotFound { .. } => (-32005, err.to_string()),
            ResolveError::InvalidTimelineItemId { .. } => (-32006, err.to_string()),
            ResolveError::InvalidNodeIndex { .. } => (-32007, err.to_string()),
            ResolveError::InvalidParameter { .. } => (-32602, err.to_string()),
            ResolveError::NotSupported { .. } => (-32601, err.to_string()),
            ResolveError::FileNotFound { .. } => (-32008, err.to_string()),
            ResolveError::PermissionDenied { .. } => (-32009, err.to_string()),
            ResolveError::Timeout { .. } => (-32010, err.to_string()),
            _ => (-32603, err.to_string()),
        };
        
        mcp::JsonRpcError::new(code, message, None)
    }
} 