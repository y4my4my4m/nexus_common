use serde::{Deserialize, Serialize};

/// Server configuration with rate limits, moderation, and file upload settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub network: NetworkConfig,
    pub rate_limits: RateLimitConfig,
    pub file_upload: FileUploadConfig,
    pub moderation: ModerationConfig,
    pub database: DatabaseConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub bind_address: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout_seconds: u64,
    pub keepalive_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub messages_per_minute: usize,
    pub requests_per_second: usize,
    pub file_uploads_per_hour: usize,
    pub registration_attempts_per_hour: usize,
    pub login_attempts_per_minute: usize,
    pub channel_joins_per_minute: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadConfig {
    pub enabled: bool,
    pub max_file_size_mb: usize,
    pub allowed_types: Vec<String>,
    pub max_files_per_user: usize,
    pub storage_path: String,
    pub cleanup_interval_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationConfig {
    pub auto_moderation_enabled: bool,
    pub blocked_words: Vec<String>,
    pub blocked_patterns: Vec<String>,
    pub auto_ban_threshold: usize,
    pub warning_threshold: usize,
    pub message_length_limit: usize,
    pub channel_creation_role: String, // "Admin", "Moderator", "User"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub backup_interval_hours: u64,
    pub backup_retention_days: u32,
    pub connection_pool_size: usize,
    pub query_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub require_secure_passwords: bool,
    pub min_password_length: usize,
    pub session_timeout_hours: u64,
    pub audit_logging_enabled: bool,
    pub ip_whitelist: Vec<String>,
    pub ip_blacklist: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                bind_address: "127.0.0.1".to_string(),
                port: 8080,
                max_connections: 1000,
                connection_timeout_seconds: 30,
                keepalive_interval_seconds: 60,
            },
            rate_limits: RateLimitConfig {
                messages_per_minute: 60,
                requests_per_second: 10,
                file_uploads_per_hour: 10,
                registration_attempts_per_hour: 5,
                login_attempts_per_minute: 5,
                channel_joins_per_minute: 20,
            },
            file_upload: FileUploadConfig {
                enabled: true,
                max_file_size_mb: 10,
                allowed_types: vec![
                    "image/png".to_string(),
                    "image/jpeg".to_string(),
                    "image/gif".to_string(),
                    "image/webp".to_string(),
                    "text/plain".to_string(),
                ],
                max_files_per_user: 100,
                storage_path: "./uploads".to_string(),
                cleanup_interval_hours: 24,
            },
            moderation: ModerationConfig {
                auto_moderation_enabled: true,
                blocked_words: vec![],
                blocked_patterns: vec![],
                auto_ban_threshold: 5,
                warning_threshold: 3,
                message_length_limit: 2000,
                channel_creation_role: "Moderator".to_string(),
            },
            database: DatabaseConfig {
                path: "nexus.db".to_string(),
                backup_interval_hours: 6,
                backup_retention_days: 30,
                connection_pool_size: 10,
                query_timeout_seconds: 30,
            },
            security: SecurityConfig {
                require_secure_passwords: true,
                min_password_length: 8,
                session_timeout_hours: 24,
                audit_logging_enabled: true,
                ip_whitelist: vec![],
                ip_blacklist: vec![],
            },
        }
    }
}

impl ServerConfig {
    /// Load configuration from file or create default
    pub fn load_or_default(path: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            match toml::from_str(&content) {
                Ok(config) => {
                    tracing::info!("Loaded server configuration from {}", path);
                    config
                }
                Err(e) => {
                    tracing::warn!("Failed to parse config file {}: {}. Using defaults.", path, e);
                    Self::default()
                }
            }
        } else {
            let default = Self::default();
            if let Err(e) = default.save(path) {
                tracing::warn!("Failed to save default config to {}: {}", path, e);
            }
            default
        }
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        tracing::info!("Saved server configuration to {}", path);
        Ok(())
    }
    
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), String> {
        if self.network.port == 0 {
            return Err("Network port cannot be 0".to_string());
        }
        
        if self.network.max_connections == 0 {
            return Err("Max connections must be greater than 0".to_string());
        }
        
        if self.rate_limits.messages_per_minute == 0 {
            return Err("Messages per minute must be greater than 0".to_string());
        }
        
        if self.moderation.message_length_limit == 0 {
            return Err("Message length limit must be greater than 0".to_string());
        }
        
        if self.security.min_password_length < 4 {
            return Err("Minimum password length must be at least 4".to_string());
        }
        
        Ok(())
    }
}

/// Client configuration for UI preferences and behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub connection: ConnectionConfig,
    pub ui: UiConfig,
    pub notifications: NotificationConfig,
    pub audio: AudioConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub auto_connect: bool,
    pub default_server: Option<String>,
    pub reconnect_attempts: usize,
    pub reconnect_delay_seconds: u64,
    pub connection_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub font_size: u16,
    pub show_timestamps: bool,
    pub show_avatars: bool,
    pub show_user_list: bool,
    pub message_grouping: bool,
    pub compact_mode: bool,
    pub sidebar_width: u16,
    pub max_message_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub show_mentions: bool,
    pub show_direct_messages: bool,
    pub show_server_invites: bool,
    pub notification_timeout_ms: u64,
    pub quiet_hours_start: Option<String>, // "22:00"
    pub quiet_hours_end: Option<String>,   // "08:00"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub enabled: bool,
    pub volume: f32,
    pub play_on_mention: bool,
    pub play_on_dm: bool,
    pub play_on_channel_message: bool,
    pub play_ui_sounds: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub image_cache_size_mb: usize,
    pub max_cached_images: usize,
    pub lazy_load_images: bool,
    pub message_render_limit: usize,
    pub scroll_buffer_size: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig {
                auto_connect: false,
                default_server: None,
                reconnect_attempts: 3,
                reconnect_delay_seconds: 5,
                connection_timeout_seconds: 10,
            },
            ui: UiConfig {
                theme: "cyberpunk".to_string(),
                font_size: 14,
                show_timestamps: true,
                show_avatars: true,
                show_user_list: true,
                message_grouping: true,
                compact_mode: false,
                sidebar_width: 30,
                max_message_history: 1000,
            },
            notifications: NotificationConfig {
                enabled: true,
                show_mentions: true,
                show_direct_messages: true,
                show_server_invites: true,
                notification_timeout_ms: 4000,
                quiet_hours_start: None,
                quiet_hours_end: None,
            },
            audio: AudioConfig {
                enabled: true,
                volume: 0.7,
                play_on_mention: true,
                play_on_dm: true,
                play_on_channel_message: false,
                play_ui_sounds: true,
            },
            performance: PerformanceConfig {
                image_cache_size_mb: 50,
                max_cached_images: 200,
                lazy_load_images: true,
                message_render_limit: 100,
                scroll_buffer_size: 500,
            },
        }
    }
}

impl ClientConfig {
    /// Load configuration from file or create default
    pub fn load_or_default(path: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            match toml::from_str(&content) {
                Ok(config) => {
                    println!("Loaded client configuration from {}", path);
                    config
                }
                Err(e) => {
                    println!("Failed to parse config file {}: {}. Using defaults.", path, e);
                    Self::default()
                }
            }
        } else {
            let default = Self::default();
            if let Err(e) = default.save(path) {
                println!("Failed to save default config to {}: {}", path, e);
            }
            default
        }
    }
    
    /// Save configuration to file
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure config directory exists
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        println!("Saved client configuration to {}", path);
        Ok(())
    }
    
    /// Get config file path
    pub fn default_path() -> String {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config/nexus/client.toml", home)
    }
}