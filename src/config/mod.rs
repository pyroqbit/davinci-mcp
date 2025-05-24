use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::Level;

/// Configuration for the DaVinci Resolve MCP Server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,
    /// Python bridge configuration
    pub python: PythonConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
}

/// Server-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server name for MCP identification
    pub name: String,
    /// Server version
    pub version: String,
    /// Server instructions/description
    pub instructions: Option<String>,
    /// Maximum number of concurrent operations
    pub max_concurrent_operations: usize,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

/// Python bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    /// Path to Python executable (if not in PATH)
    pub python_path: Option<PathBuf>,
    /// Path to DaVinci Resolve Python scripts
    pub resolve_script_path: Option<PathBuf>,
    /// Whether to initialize Python automatically
    pub auto_initialize: bool,
    /// Python bridge timeout in milliseconds
    pub bridge_timeout_ms: u64,
    /// Whether to cache Python bridge calls
    pub enable_caching: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Log format (json or pretty)
    pub format: LogFormat,
    /// Log to file path (optional)
    pub file_path: Option<PathBuf>,
    /// Whether to log to stdout
    pub stdout: bool,
    /// Whether to include line numbers
    pub include_line_numbers: bool,
    /// Whether to include thread IDs
    pub include_thread_ids: bool,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable performance metrics collection
    pub enable_metrics: bool,
    /// Thread pool size for async operations
    pub thread_pool_size: Option<usize>,
    /// Memory allocation limit in MB
    pub memory_limit_mb: Option<u64>,
    /// Enable garbage collection hints
    pub enable_gc_hints: bool,
}

/// Log level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Log format configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            python: PythonConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "davinci-resolve-mcp-rs".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            instructions: Some("High-performance Rust implementation of DaVinci Resolve MCP Server".to_string()),
            max_concurrent_operations: 10,
            timeout_seconds: 30,
        }
    }
}

impl Default for PythonConfig {
    fn default() -> Self {
        Self {
            python_path: None,
            resolve_script_path: None,
            auto_initialize: true,
            bridge_timeout_ms: 5000,
            enable_caching: true,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            file_path: None,
            stdout: true,
            include_line_numbers: true,
            include_thread_ids: false,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_metrics: false,
            thread_pool_size: None,
            memory_limit_mb: None,
            enable_gc_hints: false,
        }
    }
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| ConfigError::FileRead(e.to_string()))?;
        
        toml::from_str(&content)
            .map_err(|e| ConfigError::Parse(e.to_string()))
    }
    
    /// Save configuration to a TOML file
    pub fn to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialize(e.to_string()))?;
        
        std::fs::write(path.as_ref(), content)
            .map_err(|e| ConfigError::FileWrite(e.to_string()))?;
        
        Ok(())
    }
    
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Override with environment variables if present
        if let Ok(level) = std::env::var("DAVINCI_MCP_LOG_LEVEL") {
            config.logging.level = match level.to_lowercase().as_str() {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warn" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => LogLevel::Info,
            };
        }
        
        if let Ok(timeout) = std::env::var("DAVINCI_MCP_TIMEOUT") {
            if let Ok(timeout_val) = timeout.parse::<u64>() {
                config.server.timeout_seconds = timeout_val;
            }
        }
        
        if let Ok(python_path) = std::env::var("DAVINCI_MCP_PYTHON_PATH") {
            config.python.python_path = Some(PathBuf::from(python_path));
        }
        
        config
    }
    
    /// Merge this configuration with another, giving precedence to the other
    pub fn merge(mut self, other: Self) -> Self {
        // Simple field-by-field merge - in a real implementation,
        // you might want more sophisticated merging logic
        self.server = other.server;
        self.python = other.python;
        self.logging = other.logging;
        self.performance = other.performance;
        self
    }
}

/// Configuration-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read configuration file: {0}")]
    FileRead(String),
    
    #[error("Failed to write configuration file: {0}")]
    FileWrite(String),
    
    #[error("Failed to parse configuration: {0}")]
    Parse(String),
    
    #[error("Failed to serialize configuration: {0}")]
    Serialize(String),
} 