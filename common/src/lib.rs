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
    // Forums
    GetForums,
    CreateThread { forum_id: Uuid, title: String, content: String },
    CreatePost { thread_id: Uuid, content: String },
    // Chat
    SendChatMessage(String),
    // Moderation
    DeletePost(Uuid),
    DeleteThread(Uuid),
    // User management
    GetUserList, // Request the list of connected users
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    // Auth
    AuthSuccess(User),
    AuthFailure(String),
    // General
    Forums(Vec<Forum>),
    NewChatMessage(ChatMessage),
    Notification(String, bool), // Message, is_error
    // User management
    UserList(Vec<User>), // List of connected users
    UserJoined(User),    // A user joined
    UserLeft(Uuid),      // A user left (by id)
}


// Initial data creation
pub fn create_initial_forums() -> Vec<Forum> {
    let system_user = User {
        id: Uuid::new_v4(),
        username: "system".to_string(),
        color: Color::Red,
        role: UserRole::Admin,
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