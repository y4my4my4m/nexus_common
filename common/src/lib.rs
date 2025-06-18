// common/src/lib.rs

use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- Serde helper for ratatui::Color ---
// *** FIX 1: MOVE THIS TO THE TOP LEVEL ***
#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
pub enum ColorDef {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

// *** FIX 2: CREATE A WRAPPER FOR DIRECT SERIALIZATION ***
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SerializableColor(#[serde(with = "ColorDef")] pub Color);


// --- User & Role Management ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserRole {
    User,
    Moderator,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(with = "ColorDef")]
    pub color: Color,
    pub role: UserRole,
    pub profile_pic: Option<String>,
    pub cover_banner: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    #[serde(rename = "password_hash")]
    pub hash: String,
    #[serde(with = "ColorDef")]
    pub color: Color,
    pub role: UserRole,
    // Profile fields
    pub bio: Option<String>,
    pub url1: Option<String>,
    pub url2: Option<String>,
    pub url3: Option<String>,
    pub location: Option<String>,
    pub profile_pic: Option<String>,
    pub cover_banner: Option<String>,
}

// --- Data Structures ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Forum {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thread {
    pub id: Uuid,
    pub title: String,
    pub author: User,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub author: User,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub author: String,
    pub content: String,
    #[serde(with = "ColorDef")]
    pub color: Color,
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
    pub sent_by: Uuid,
    pub timestamp: i64,
    pub content: String,
    // --- Added fields for author info ---
    pub author_username: String,
    #[serde(with = "ColorDef")]
    pub author_color: Color,
    pub author_profile_pic: Option<String>, // base64 or URL
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
    // *** FIX 2: USE THE WRAPPER STRUCT HERE ***
    UpdateColor(SerializableColor),
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
    CreateThread { forum_id: Uuid, title: String, content: String },
    CreatePost { thread_id: Uuid, content: String },
    // Chat
    SendDirectMessage { to: Uuid, content: String },
    SendChannelMessage { channel_id: Uuid, content: String },
    // Moderation
    DeletePost(Uuid),
    DeleteThread(Uuid),
    // User management
    GetUserList, // Request the list of connected users
    GetProfile { user_id: Uuid },
    GetServers, // Request all servers the user is a member of
    // --- CHANNEL MESSAGE FETCH ---
    GetChannelMessages { channel_id: Uuid },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    // Auth
    AuthSuccess(User),
    AuthFailure(String),
    // General
    Forums(Vec<Forum>),
    NewChatMessage(ChatMessage),
    DirectMessage { from: User, content: String },
    MentionNotification { from: User, content: String },
    Notification(String, bool), // Message, is_error
    // User management
    UserList(Vec<User>), // List of connected users
    UserJoined(User),    // A user joined
    UserLeft(Uuid),      // A user left (by id)
    Profile(UserProfile),
    UserUpdated(User), // Broadcast when a user updates their profile
    Servers(Vec<Server>), // List of servers and their channels
    NewChannelMessage(ChannelMessage),
    // --- CHANNEL MESSAGE FETCH ---
    ChannelMessages { channel_id: Uuid, messages: Vec<ChannelMessage> },
}


// Initial data creation
pub fn create_initial_forums() -> Vec<Forum> {
    let system_user = User {
        id: Uuid::new_v4(),
        username: "system".to_string(),
        color: Color::Red,
        role: UserRole::Admin,
        profile_pic: Some("system.png".to_string()),
        cover_banner: Some("system_banner.png".to_string()),
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
                    posts: vec![ Post {
                        id: Uuid::new_v4(),
                        author: system_user.clone(),
                        content: "I've been probing their new Aegis system. It's tough.".to_string(),
                    }],
                },
            ],
        },
    ]
}