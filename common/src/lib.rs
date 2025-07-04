// common/src/lib.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod config;
pub use config::{ServerConfig, ClientConfig};

// Simple color representation that works for both client and server
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserColor(pub String);

impl UserColor {
    pub fn new(color: impl Into<String>) -> Self {
        Self(color.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for UserColor {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for UserColor {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

// For client compatibility with ratatui
#[cfg(feature = "ratatui")]
impl From<ratatui::style::Color> for UserColor {
    fn from(color: ratatui::style::Color) -> Self {
        match color {
            ratatui::style::Color::Rgb(r, g, b) => Self(format!("#{:02X}{:02X}{:02X}", r, g, b)),
            ratatui::style::Color::Red => Self("Red".to_string()),
            ratatui::style::Color::Green => Self("Green".to_string()),
            ratatui::style::Color::Blue => Self("Blue".to_string()),
            ratatui::style::Color::Yellow => Self("Yellow".to_string()),
            ratatui::style::Color::Cyan => Self("Cyan".to_string()),
            ratatui::style::Color::Magenta => Self("Magenta".to_string()),
            ratatui::style::Color::White => Self("White".to_string()),
            ratatui::style::Color::Black => Self("Black".to_string()),
            _ => Self("Cyan".to_string()), // Default
        }
    }
}

#[cfg(feature = "ratatui")]
impl From<UserColor> for ratatui::style::Color {
    fn from(user_color: UserColor) -> ratatui::style::Color {
        if user_color.0.starts_with('#') && user_color.0.len() == 7 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&user_color.0[1..3], 16),
                u8::from_str_radix(&user_color.0[3..5], 16),
                u8::from_str_radix(&user_color.0[5..7], 16),
            ) {
                return ratatui::style::Color::Rgb(r, g, b);
            }
        }
        
        match user_color.0.as_str() {
            "Red" => ratatui::style::Color::Red,
            "Green" => ratatui::style::Color::Green,
            "Blue" => ratatui::style::Color::Blue,
            "Yellow" => ratatui::style::Color::Yellow,
            "Cyan" => ratatui::style::Color::Cyan,
            "Magenta" => ratatui::style::Color::Magenta,
            "White" => ratatui::style::Color::White,
            "Black" => ratatui::style::Color::Black,
            _ => ratatui::style::Color::Cyan, // Default
        }
    }
}

// --- User & Role Management ---

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UserRole {
    User,
    Moderator,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Connected,
    Away,
    Busy,
    Offline,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub color: UserColor,
    pub role: UserRole,
    pub profile_pic: Option<String>,
    pub cover_banner: Option<String>,
    pub status: UserStatus,
}

/// Complete user profile with all fields (for profile editing/viewing)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub hash: String, // Password hash (server-side only)
    pub color: UserColor,
    pub role: UserRole,
    pub bio: Option<String>,
    pub url1: Option<String>,
    pub url2: Option<String>,
    pub url3: Option<String>,
    pub location: Option<String>,
    pub profile_pic: Option<String>,
    pub cover_banner: Option<String>,
}

/// Lightweight user info without profile images - use this for lists, mentions, etc.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub color: UserColor,
    pub role: UserRole,
    pub status: UserStatus,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            color: user.color,
            role: user.role,
            status: user.status,
        }
    }
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            color: user.color.clone(),
            role: user.role,
            status: user.status, // Now UserStatus implements Copy, so no clone needed
        }
    }
}

// --- Data Structures ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Forum {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

/// Lightweight forum structure for initial loading (no profile images)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForumLightweight {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub threads: Vec<ThreadLightweight>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thread {
    pub id: Uuid,
    pub title: String,
    pub author: User,
    pub posts: Vec<Post>,
    pub timestamp: i64,
}

/// Lightweight thread structure for initial loading (no profile images)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadLightweight {
    pub id: Uuid,
    pub title: String,
    pub author: UserInfo,
    pub posts: Vec<PostLightweight>,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub author: User,
    pub content: String,
    pub timestamp: i64,
    pub reply_to: Option<Uuid>,
}

/// Lightweight post structure for initial loading (no profile images)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostLightweight {
    pub id: Uuid,
    pub author: UserInfo,
    pub content: String,
    pub timestamp: i64,
    pub reply_to: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub author: String,
    pub content: String,
    pub color: UserColor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub invite_code: Option<String>,
    pub icon: Option<String>, // base64
    pub banner: Option<String>, // base64
    pub owner: Uuid,
    pub mods: Vec<Uuid>,
    pub userlist: Vec<Uuid>,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub description: String,
    pub permissions: ChannelPermissions,
    pub userlist: Vec<Uuid>,
    pub messages: Vec<ChannelMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelPermissions {
    pub can_read: Vec<Uuid>,
    pub can_write: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub sent_by: Uuid, // This is the author ID - frontend will look up user info
    pub timestamp: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectMessage {
    pub id: Uuid,
    pub from: Uuid, // This is the author ID - frontend will look up user info
    pub to: Uuid,
    pub timestamp: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NotificationType {
    ThreadReply,
    DM,
    Announcement,
    Mention,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notif_type: NotificationType,
    pub related_id: Uuid,
    pub created_at: i64,
    pub read: bool,
    pub extra: Option<String>,
}

// --- Server Invites ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInvite {
    pub id: Uuid,
    pub from_user: User,
    pub to_user_id: Uuid,
    pub server: Server,
    pub timestamp: i64,
    pub status: ServerInviteStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServerInviteStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}


// --- Network Protocol Definitions ---

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    // Auth
    Register { username: String, password: String },
    Login { username: String, password: String },
    Logout,
    // User
    UpdatePassword(String),
    UpdateColor(UserColor), // Changed from SerializableColor to UserColor
    UpdateProfile {
        bio: Option<String>,
        url1: Option<String>,
        url2: Option<String>,
        url3: Option<String>,
        location: Option<String>,
        profile_pic: Option<String>,
        cover_banner: Option<String>,
    },
    // Forums
    GetForums,
    CreateForum { name: String, description: String },
    DeleteForum { forum_id: Uuid },
    CreateThread { forum_id: Uuid, title: String, content: String },
    CreatePost { thread_id: Uuid, content: String },
    CreatePostReply { thread_id: Uuid, content: String, reply_to: Uuid }, // New: Reply to specific post
    // Chat
    SendDirectMessage { to: Uuid, content: String },
    SendChannelMessage { channel_id: Uuid, content: String },
    // Server invites
    SendServerInvite { to_user_id: Uuid, server_id: Uuid },
    RespondToServerInvite { invite_id: Uuid, accept: bool },
    // New: Accept/decline invite from a specific user (for DM commands)
    AcceptServerInviteFromUser { from_user_id: Uuid },
    DeclineServerInviteFromUser { from_user_id: Uuid },
    // Moderation
    DeletePost(Uuid),
    DeleteThread(Uuid),
    // User management
    GetUserList, // Request the list of connected users
    GetProfile { user_id: Uuid },
    GetServers, // Request all servers the user is a member of
    // --- ENHANCED PAGINATION SUPPORT ---
    GetChannelMessagesPaginated { 
        channel_id: Uuid, 
        cursor: PaginationCursor,
        limit: Option<usize>,
        direction: PaginationDirection,
    },
    GetDirectMessagesPaginated { 
        user_id: Uuid, 
        cursor: PaginationCursor,
        limit: Option<usize>,
        direction: PaginationDirection,
    },
    // --- LEGACY COMPATIBILITY ---
    GetChannelMessages { channel_id: Uuid, before: Option<i64> },
    GetChannelUserList { channel_id: Uuid },
    GetDMUserList, // Request list of users you have DMs with
    GetDirectMessages { user_id: Uuid, before: Option<i64> }, // Fetch DMs with a user, paginated by timestamp
    // --- NOTIFICATIONS ---
    GetNotifications { before: Option<i64> },
    MarkNotificationRead { notification_id: Uuid },
    // --- CACHE MANAGEMENT ---
    InvalidateImageCache { keys: Vec<String> },
    GetCacheStats,
    // Profile picture requests (for efficient loading)
    GetUserAvatars { user_ids: Vec<Uuid> },
}

/// Pagination cursor for network protocol
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PaginationCursor {
    Timestamp(i64),
    Offset(usize),
    Start,
}

/// Pagination direction for network protocol
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PaginationDirection {
    Forward,
    Backward,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    // Auth
    AuthSuccess(User),
    AuthFailure(String),
    // General
    Forums(Vec<Forum>),
    ForumsLightweight(Vec<ForumLightweight>), // Lightweight version without profile images
    NewChatMessage(ChatMessage),
    DirectMessage(DirectMessage),
    MentionNotification { from: User, content: String },
    ForumReplyNotification { thread_id: Uuid, from_username: String, message: String, from_user_profile_pic: Option<String> },
    Notification(String, bool), // Message, is_error
    // Server invites
    ServerInviteReceived(ServerInvite),
    ServerInviteResponse { invite_id: Uuid, accepted: bool, user: User },
    // User management
    UserList(Vec<User>), // List of connected users
    UserJoined(User),    // A user joined
    UserLeft(Uuid),      // A user left (by id)
    Profile(UserProfile),
    UserUpdated(User), // Broadcast when a user updates their profile
    Servers(Vec<Server>), // List of servers and their channels
    NewChannelMessage(ChannelMessage),
    // --- ENHANCED PAGINATION RESPONSES ---
    ChannelMessagesPaginated { 
        channel_id: Uuid, 
        messages: Vec<ChannelMessage>, 
        has_more: bool,
        next_cursor: Option<PaginationCursor>,
        prev_cursor: Option<PaginationCursor>,
        total_count: Option<usize>,
    },
    DirectMessagesPaginated { 
        user_id: Uuid, 
        messages: Vec<DirectMessage>, 
        has_more: bool,
        next_cursor: Option<PaginationCursor>,
        prev_cursor: Option<PaginationCursor>,
        total_count: Option<usize>,
    },
    // --- LEGACY COMPATIBILITY ---
    ChannelMessages { channel_id: Uuid, messages: Vec<ChannelMessage>, history_complete: bool },
    ChannelUserList { channel_id: Uuid, users: Vec<User> },
    DMUserList(Vec<User>), // List of users you have DMs with
    DirectMessages { user_id: Uuid, messages: Vec<DirectMessage>, history_complete: bool },
    // --- NOTIFICATIONS ---
    Notifications { notifications: Vec<Notification>, history_complete: bool },
    // --- CACHE MANAGEMENT ---
    CacheStats { 
        total_entries: usize, 
        total_size_mb: f64, 
        hit_ratio: f64,
        expired_entries: usize,
    },
    ImageCacheInvalidated { keys: Vec<String> },
    UserAvatars { avatars: Vec<(Uuid, Option<String>)> }, // user_id, profile_pic
    // --- PERFORMANCE METRICS ---
    PerformanceMetrics {
        query_time_ms: u64,
        cache_hit_rate: f64,
        message_count: usize,
    },
}


// Initial data creation
pub fn create_initial_forums() -> Vec<Forum> {
    let system_user = User {
        id: Uuid::new_v4(),
        username: "system".to_string(),
        color: UserColor::new("Red"),
        role: UserRole::Admin,
        profile_pic: Some("system.png".to_string()),
        cover_banner: Some("system_banner.png".to_string()),
        status: UserStatus::Connected, // Default to connected
    };
    vec![
        Forum {
            id: Uuid::new_v4(),
            name: "Decompiling Corporate ICE".to_string(),
            description: "Tips and tricks for getting past the big boys' security.".to_string(),
            threads: vec![
                Thread {
                    id: Uuid::new_v4(),
                    title: "Militech's 'Aegis' Firewall - Any exploits?".to_string(),
                    author: system_user.clone(),
                    timestamp: 1633072800,
                    posts: vec![ Post {
                        id: Uuid::new_v4(),
                        author: system_user.clone(),
                        content: "I've been probing their new Aegis system. It's tough.".to_string(),
                        timestamp: 1633072800,
                        reply_to: None, // No parent post
                    }],
                },
            ],
        },
    ]
}