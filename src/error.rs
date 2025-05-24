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
    
    #[error("Tool not found: {name}")]
    ToolNotFound { name: String },
    
    #[error("Invalid timeline item ID: {id}")]
    InvalidTimelineItemId { id: String },
    
    #[error("Invalid node index: {index}")]
    InvalidNodeIndex { index: i32 },
    
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
impl From<ResolveError> for rmcp::Error {
    fn from(err: ResolveError) -> Self {
        match err {
            ResolveError::NotRunning => rmcp::Error::invalid_request(err.to_string(), None),
            ResolveError::ProjectNotFound { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::TimelineNotFound { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::MediaNotFound { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::BinNotFound { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::ToolNotFound { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::InvalidTimelineItemId { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::InvalidNodeIndex { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::InvalidParameter { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::NotSupported { .. } => rmcp::Error::internal_error(err.to_string(), None),
            ResolveError::FileNotFound { .. } => rmcp::Error::invalid_params(err.to_string(), None),
            ResolveError::PermissionDenied { .. } => rmcp::Error::internal_error(err.to_string(), None),
            ResolveError::Timeout { .. } => rmcp::Error::internal_error(err.to_string(), None),
            _ => rmcp::Error::internal_error(err.to_string(), None),
        }
    }
} 