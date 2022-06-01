use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleContent {
    pub id: i32,
    pub r#type: i32,
    pub title: String,
    pub cover_image: String,
    pub up_date: String,
}

/// this struct use for display a item on home page.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Content {
    pub id: i32,
    pub r#type: i32,
    pub title: String,
    pub source: String,
    pub author: SimpleUser,
    pub topic: i32,
    pub description: String,
    pub cover_image: String,
    pub up_date: String,
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Topic {
    pub id: i32,
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleUser {
    pub id: i32,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub gender: String,
    pub reg_date: String,
    pub introduction: String,
}

#[derive(Clone, PartialEq)]
pub struct PlayerBoxStatus {
    pub display: bool,
    pub pause: bool,
    pub current: Option<Content>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct OperResult {
    pub status: String,
    pub message: String
}

#[derive(Deserialize, Clone)]
pub struct AuthInfo {
    pub token: String,
    pub expire: i64,
}