use ratatui::style::Color;
use serde::{Deserialize, Serialize};

// --- Data Structures (serializable) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Forum {
    pub name: String,
    pub description: String,
    pub threads: Vec<Thread>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thread {
    pub title: String,
    pub author: String,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub author: String,
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
    SetUsername(String),
    GetForums,
    AddThread {
        forum_idx: usize,
        thread: Thread,
    },
    AddPost {
        forum_idx: usize,
        thread_idx: usize,
        post: Post,
    },
    SendChatMessage(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Forums(Vec<Forum>),
    NewChatMessage(ChatMessage),
}

// --- Serde helper for ratatui::Color ---
// This is needed because Color doesn't implement Serialize/Deserialize by default
#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
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

// --- Mock Data Creation (for server's first run) ---
pub fn create_initial_forums() -> Vec<Forum> {
    vec![
        // ... (same as before, just using the new serializable structs)
        Forum {
            name: "Decompiling Corporate ICE".to_string(),
            description: "Tips and tricks for getting past the big boys' security.".to_string(),
            threads: vec![
                Thread {
                    title: "Militech's 'Aegis' Firewall - Any exploits?".to_string(),
                    author: "jack_h.k".to_string(),
                    posts: vec![
                        Post { author: "jack_h.k".to_string(), content: "I've been probing their new Aegis system. It's tough. The outer layer seems to use quantum entanglement for key generation. Standard brute-forcing is useless.".to_string() },
                        Post { author: "DataWitch".to_string(), content: "Heard that. You need to look for social engineering vectors. The human element is always the weakest link. Check their janitorial staff's public data.".to_string() },
                    ],
                },
            ],
        },
        Forum {
            name: "Black Market Bazaar".to_string(),
            description: "Trade gear, software, and information. No feds.".to_string(),
            threads: vec![
                Thread {
                    title: "[WTS] Kiroshi Optics (Gen 3)".to_string(),
                    author: "fixer_x".to_string(),
                    posts: vec![
                        Post { author: "fixer_x".to_string(), content: "Got a fresh pair, clean serial. 5000 eddies. No lowballers, I know what I have.".to_string() },
                    ],
                },
            ],
        },
    ]
}