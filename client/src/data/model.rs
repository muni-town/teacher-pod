use serde::{Deserialize, Serialize};

/// this struct use for display a item on home page.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleArticle {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub author_id: i32,
    pub create_date: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    pub id: i32,
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleUser {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub email: String,
}

#[derive(Clone, PartialEq)]
pub struct PlayerBoxStatus {
    pub display: bool,
    pub pause: bool,
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