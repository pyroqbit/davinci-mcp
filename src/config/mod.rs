use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the DaVinci Resolve MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,
    /// Logging configuration  
    pub logging: LoggingConfig,
    /// DaVinci Resolve specific settings
    pub resolve: ResolveConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server port (not used for stdio MCP)
    pub port: u16,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Request timeout in seconds
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Log format (json, pretty)
    pub format: String,
    /// Log to file path (optional)
    pub file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveConfig {
    /// Auto-connect to DaVinci Resolve on startup
    pub auto_connect: bool,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Retry attempts for failed operations
    pub retry_attempts: u32,
    /// Default project settings
    pub default_project: DefaultProjectConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultProjectConfig {
    /// Default frame rate
    pub frame_rate: String,
    /// Default resolution width
    pub width: u32,
    /// Default resolution height  
    pub height: u32,
    /// Default color space
    pub color_space: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            resolve: ResolveConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            max_connections: 100,
            timeout: 30,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            file: None,
        }
    }
}

impl Default for ResolveConfig {
    fn default() -> Self {
        Self {
            auto_connect: true,
            connection_timeout: 10,
            retry_attempts: 3,
            default_project: DefaultProjectConfig::default(),
        }
    }
}

impl Default for DefaultProjectConfig {
    fn default() -> Self {
        Self {
            frame_rate: "24".to_string(),
            width: 1920,
            height: 1080,
            color_space: "Rec.709".to_string(),
        }
    }
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for development/testing
    pub fn development() -> Self {
        let mut config = Self::default();
        config.logging.level = "debug".to_string();
        config.resolve.auto_connect = false; // Don't auto-connect in dev mode
        config
    }

    /// Create configuration for production
    pub fn production() -> Self {
        let mut config = Self::default();
        config.logging.level = "warn".to_string();
        config.logging.format = "json".to_string();
        config
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate log level
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            return Err(format!("Invalid log level: {}", self.logging.level));
        }

        // Validate log format
        let valid_formats = ["json", "pretty"];
        if !valid_formats.contains(&self.logging.format.as_str()) {
            return Err(format!("Invalid log format: {}", self.logging.format));
        }

        // Validate frame rate
        let frame_rate: Result<f64, _> = self.resolve.default_project.frame_rate.parse();
        if frame_rate.is_err() {
            return Err(format!("Invalid frame rate: {}", self.resolve.default_project.frame_rate));
        }

        Ok(())
    }
} 