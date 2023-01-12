use crate::schema::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub phone: String,
    pub web_socket_session: String,
    pub created_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
pub struct Message {
    pub id: String,
    pub room_id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewMessage {
    pub room_id: String,
    pub user_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Eq, Hash, PartialEq)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub phone: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConversation {
    pub user_id: String,
    pub room_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub room: Room,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoomToUser {
    pub room: String,
    pub user: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct RoomToUser {
    pub id: String,
    pub room: String,
    pub user: String,
}
