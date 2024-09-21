use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPreview {
    pub id: u32,
    pub last_message: String,
    pub user: crate::models::User,
    pub last_message_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub chat_room_id: i32,
    pub user_id: i32,
    pub content: String,
    // pub sent_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: u32,
    pub name: Option<String>, // For group chat rooms, this could be None for one-on-one chats
    pub is_group: bool,
    // pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRoomMember {
    pub id: u32,
    pub user_id: i32,
    pub chat_room_id: i32,
    // pub created_at: chrono::NaiveDateTime,
}
